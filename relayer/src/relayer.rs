use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, FixedBytes, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    sol_types::SolCall,
    transports::RpcError,
};
use bindings::full_relay_with_verify::{
    FullRelayWithVerify as BitcoinRelay,
    FullRelayWithVerify::FullRelayWithVerifyInstance as BitcoinRelayInstance,
};
use bitcoin::{block::Header as BitcoinHeader, consensus, hashes::Hash, BlockHash};
use eyre::{eyre, Result};
use reqwest::{Client, Url};
use serde_json::Value;
use std::time::Duration;
use utils::EsploraClient;

const HEADERS_PER_BATCH: u32 = 5;

type ContractProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

fn serialize_headers(headers: &[RelayHeader]) -> Result<Vec<u8>> {
    Ok(headers.iter().flat_map(|header| consensus::serialize(&header.header)).collect())
}

#[derive(Clone)]
struct RelayHeader {
    hash: BlockHash,
    header: BitcoinHeader,
    height: u32,
}

impl RelayHeader {
    fn serialize_hash(&self) -> FixedBytes<32> {
        FixedBytes::from_slice(self.hash.as_byte_array())
    }

    fn serialize_header(&self) -> Bytes {
        Bytes::from(consensus::serialize(&self.header))
    }
}

#[derive(Clone)]
pub struct Relayer {
    contract: BitcoinRelayInstance<ContractProvider>,
    esplora_client: EsploraClient,
}

// https://github.com/summa-tx/relays/tree/master/maintainer/maintainer/header_forwarder
impl Relayer {
    pub fn new(
        bob_url: Url,
        relay_address: Address,
        esplora_client: EsploraClient,
        wallet: EthereumWallet,
    ) -> Self {
        let provider = ProviderBuilder::new().wallet(wallet).connect_http(bob_url);
        let contract = BitcoinRelay::new(relay_address, provider);

        Self { contract, esplora_client }
    }

    pub fn read_only(bob_url: Url, relay_address: Address, esplora_client: EsploraClient) -> Self {
        let signer = PrivateKeySigner::random();
        let wallet = EthereumWallet::from(signer);

        Self::new(bob_url, relay_address, esplora_client, wallet)
    }

    /// Helper function to deploy relay contract.
    pub async fn deploy_contract(
        rpc_url: &Url,
        wallet: &EthereumWallet,
        esplora_client: &EsploraClient,
        genesis_height: u32,
        period_start_height: u32,
    ) -> Result<Address> {
        let provider = ProviderBuilder::new().wallet(wallet.clone()).connect_http(rpc_url.clone());

        let period_start_block_hash = esplora_client
            .get_block_hash(period_start_height)
            .await
            .map_err(|_| eyre!("Couldn't find block hash at period_start_height"))?;
        let genesis_block_hash = esplora_client
            .get_block_hash(genesis_height)
            .await
            .map_err(|_| eyre!("Couldn't find block hash at genesis_height"))?;
        let genesis_block_header =
            esplora_client
                .get_raw_block_header(&genesis_block_hash)
                .await
                .map_err(|_| eyre!("Couldn't find block header at genesis_height"))?;

        let contract = BitcoinRelay::deploy(
            provider.clone(),
            Bytes::from(genesis_block_header.clone()),
            U256::from(genesis_height),
            FixedBytes::new(period_start_block_hash.to_byte_array()),
        )
        .await?;

        Ok(*contract.address())
    }

    pub fn contract_address(&self) -> Address {
        *self.contract.address()
    }

    /// Returns the height relayed to the smart contract so far.
    pub async fn relayed_height(&self) -> Result<u32> {
        let relayer_blockhash = self.contract.getBestKnownDigest().call().await?;
        let relayed_height: u32 =
            self.contract.findHeight(relayer_blockhash).call().await?.try_into().unwrap();

        Ok(relayed_height)
    }

    /// Returns the block hash of the most recent relayed block.
    pub async fn best_blockhash(&self) -> Result<BlockHash> {
        Ok(self
            .contract
            .getBestKnownDigest()
            .call()
            .await
            .map(|bytes| BlockHash::from_byte_array(bytes.into()))?)
    }

    /// Returns true if the block with this hash was relayed to the contract.
    pub async fn has_relayed(&self, blockhash: BlockHash) -> Result<bool> {
        let result = self.contract.findHeight(blockhash.to_byte_array().into()).call().await;

        match result {
            Ok(_) => Ok(true),
            Err(alloy::contract::Error::TransportError(RpcError::ErrorResp(_e))) => {
                // If the block is not relayed, the findHeight call reverts.
                Ok(false)
            }
            Err(e) => Err(e)?, // something else went wrong (e.g. network issues)
        }
    }

    async fn latest_common_height(&self) -> Result<u32> {
        // Start at the tip of the relayed chain, then move back until we find a block that matches
        // bitcoin chain. We do it like this because calling esplora.get_block_hash for a
        // block in a fork will fail.
        let mut height = self.relayed_height().await?;

        loop {
            let actual_hash = self.esplora_client.get_block_hash(height).await?;
            let is_relayed = self.has_relayed(actual_hash).await?;

            if is_relayed {
                return Ok(height);
            }

            tracing::info!("Found fork: {actual_hash} at height {height}");
            if height == 0 {
                return Err(eyre!("No common height found before reaching 0"));
            }
            height -= 1;
        }
    }

    pub async fn run_once(&self) -> Result<()> {
        let latest_height = self.latest_common_height().await?;
        let headers: Vec<RelayHeader> = self.pull_headers(latest_height).await?;
        self.push_headers(headers).await?;
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let mut latest_height = self.latest_common_height().await?;

        loop {
            let headers = self.pull_headers(latest_height).await?;

            latest_height += headers.len() as u32;

            self.push_headers(headers).await?;

            tokio::time::sleep(if self.is_regtest() {
                Duration::from_millis(150)
            } else {
                Duration::from_secs(15)
            })
            .await;
        }
    }

    async fn pull_headers(&self, mut latest_height: u32) -> Result<Vec<RelayHeader>> {
        latest_height += 1;

        // Larger batches are more convenient for integration tests
        let headers_per_patch =
            if self.is_regtest() { HEADERS_PER_BATCH * 10 } else { HEADERS_PER_BATCH };

        // let futures = (latest_height..latest_height + headers_per_patch )
        //     .map(|height| async move {
        //         tracing::debug!("fetching height {}", height);
        //         let hash = self.esplora_client.get_block_hash(height).await?;
        //         let header = self.esplora_client.get_block_header(&hash).await?;
        //         Ok(RelayHeader { hash, header, height }) as Result<RelayHeader>
        //     })
        //     .collect::<Vec<_>>();
        // Ok(try_join_all(futures).await?)

        let bitcoin_height = self.esplora_client.get_chain_height().await?;
        tracing::debug!("bitcoin_height {bitcoin_height}");

        let mut relay_headers = Vec::new();
        for height in latest_height..(latest_height + headers_per_patch).min(bitcoin_height + 1) {
            tracing::debug!("fetching height {}", height);
            let hash = self.esplora_client.get_block_hash(height).await?;
            let header = self.esplora_client.get_block_header(&hash).await?;
            relay_headers.push(RelayHeader { hash, header, height });
        }
        Ok(relay_headers)
    }

    async fn push_headers(&self, headers: Vec<RelayHeader>) -> Result<()> {
        if headers.is_empty() {
            tracing::debug!("No headers to push..");

            return Ok(());
        };

        let start_mod = headers.first().unwrap().height % 2016;
        let end_header = headers.last().unwrap().clone();
        let end_mod = end_header.height % 2016;

        // difficulty change first
        if start_mod == 0 {
            tracing::debug!("difficulty change first");
            self.add_diff_change(headers).await?;
        }
        // spanning difficulty change
        else if start_mod > end_mod {
            tracing::debug!("spanning difficulty change");
            let (pre_change, post_change): (Vec<RelayHeader>, Vec<RelayHeader>) =
                headers.into_iter().partition(|header| header.height % 2016 >= start_mod);

            if !pre_change.is_empty() {
                self.add_headers(pre_change).await?;
            }

            if !post_change.is_empty() {
                self.add_diff_change(post_change).await?;
            }
        }
        // no difficulty change
        else {
            tracing::debug!("adding headers");
            self.add_headers(headers).await?;
        }

        tracing::debug!("updating head");
        self.update_best_digest(end_header).await?;

        tracing::debug!(
            "Relayed until height {}, with hash {}",
            self.relayed_height().await?,
            self.best_blockhash().await?
        );

        Ok(())
    }

    async fn add_diff_change(&self, headers: Vec<RelayHeader>) -> Result<()> {
        let epoch_start = headers.first().unwrap().height - 2016;
        let old_start = self.esplora_client.get_raw_block_header_at_height(epoch_start).await?;
        let old_end =
            self.esplora_client.get_raw_block_header_at_height(epoch_start + 2015).await?;

        let receipt = self
            .contract
            .addHeadersWithRetarget(
                Bytes::from(old_start),
                Bytes::from(old_end),
                Bytes::from(serialize_headers(&headers)?),
            )
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }

    async fn add_headers(&self, headers: Vec<RelayHeader>) -> Result<()> {
        let latest_digest = headers.first().unwrap().header.prev_blockhash;
        let anchor = self.esplora_client.get_raw_block_header(&latest_digest).await?;

        let receipt = self
            .contract
            .addHeaders(Bytes::from(anchor), Bytes::from(serialize_headers(&headers)?))
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }

    async fn find_lca(
        &self,
        new_best: &RelayHeader,
        current_best: RelayHeader,
    ) -> Result<RelayHeader> {
        let mut ancestor = current_best;

        for _ in 0..20 {
            let is_ancestor = self
                .contract
                .isAncestor(ancestor.serialize_hash(), new_best.serialize_hash(), U256::from(240))
                .call()
                .await?;

            if is_ancestor {
                return Ok(ancestor);
            }

            ancestor = RelayHeader {
                hash: ancestor.header.prev_blockhash,
                header: self
                    .esplora_client
                    .get_block_header(&ancestor.header.prev_blockhash)
                    .await?,
                height: ancestor.height.saturating_sub(1),
            };
        }

        Err(eyre!("Could not find ancenstor"))
    }

    async fn update_best_digest(&self, new_best: RelayHeader) -> Result<()> {
        let current_best = self.get_heaviest_relayed_block_header().await?;

        let ancestor = self.find_lca(&new_best, current_best.clone()).await?;
        let delta = new_best.height - ancestor.height + 1;

        let receipt = self
            .contract
            .markNewHeaviest(
                ancestor.serialize_hash(),
                current_best.serialize_header(),
                new_best.serialize_header(),
                U256::from(delta),
            )
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }

    async fn get_heaviest_relayed_block_header(&self) -> Result<RelayHeader> {
        if self.is_regtest() {
            self.get_heaviest_relayed_block_header_regtest().await
        } else {
            self.get_heaviest_relayed_block_header_goldsky().await
        }
    }

    /// Fetch the block header from the contract. We used to fetch from esplora but that would
    /// fail if there was a fork. This function is currently a bit over engineered - it uses
    /// a subgraph to find the tx that submitted the heaviest block, then takes the blockheader
    /// from its calldata. It would have been a lot easier if the smart contract were to store
    /// the blockheader directly - something we might do in the future. That would come at the
    /// cost of additional gas usage though.
    async fn get_heaviest_relayed_block_header_goldsky(&self) -> Result<RelayHeader> {
        let relayer_blockhash = self.contract.getBestKnownDigest().call().await?;

        let query = format!(
            r#"
            query MyQuery {{
                newTips(
                    first: 1
                    orderBy: block_number
                    orderDirection: desc
                    where: {{to: "{relayer_blockhash}"}}
                ) {{
                    transactionHash_
                }}
            }}
        "#
        );

        let res: Value = Client::new()
            .post("https://api.goldsky.com/api/public/project_clto8zgmd1jbw01xig1ge1u0h/subgraphs/Relay-sepolia/1.0.0/gn")
            .json(&serde_json::json!({ "query": query }))
            .send()
            .await?
            .json()
            .await?;

        let txid = res["data"]["newTips"]
            .as_array()
            .and_then(|x| x.first())
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("transactionHash_"))
            .and_then(|x| x.as_str())
            .ok_or(eyre!("No events in the subgraph"))?
            .to_string();

        let txid: [u8; 32] = alloy::hex::decode(txid)?.try_into().unwrap();

        let tx = self.contract.provider().get_transaction_by_hash(txid.into()).await?.unwrap();
        use alloy::consensus::Transaction;

        let input = tx.as_ref().input();

        use bindings::full_relay_with_verify::FullRelayWithVerify::markNewHeaviestCall;

        let decoded = markNewHeaviestCall::abi_decode(input)?;
        let header: bitcoin::block::Header = bitcoin::consensus::deserialize(&decoded._newBest.0)?;

        let height = self.contract.findHeight(relayer_blockhash).call().await?;
        let hash = bitcoin::BlockHash::from_slice(&relayer_blockhash.0)?;
        Ok(RelayHeader { hash, header, height: height.try_into()? })
    }

    async fn get_heaviest_relayed_block_header_regtest(&self) -> Result<RelayHeader> {
        let current_best_digest_raw = self.contract.getBestKnownDigest().call().await?;
        let current_best_digest = BlockHash::from_byte_array(current_best_digest_raw.0);
        Ok(RelayHeader {
            hash: current_best_digest,
            header: self.esplora_client.get_block_header(&current_best_digest).await?,
            height: self
                .contract
                .findHeight(current_best_digest_raw)
                .call()
                .await?
                .try_into()
                .expect("height larger than u32::MAX"),
        })
    }

    fn is_regtest(&self) -> bool {
        self.contract.provider().client().is_local()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::hex;

    #[tokio::test]
    async fn test_has_relayed() -> Result<()> {
        let relayer = Relayer::read_only(
            "https://bob-sepolia.rpc.gobob.xyz/".parse()?,
            "0xaAD39528eB8b3c70b613C442F351610969974fDF".parse()?,
            EsploraClient::new(bitcoin::Network::Signet)?,
        );

        assert!(!relayer.has_relayed(BlockHash::from_slice(&[1u8; 32])?).await?);
        assert!(
            relayer
                .has_relayed(BlockHash::from_slice(&hex::decode(
                    "0x915c9fffe077970ee032ed8be0c6953fe2a4ab9827ca151e4977a6d72a010000"
                )?)?)
                .await?
        );

        Ok(())
    }

    #[tokio::test]
    #[ignore] // Run this manually with anvil --fork-url wss://bob-sepolia.rpc.gobob.xyz --fork-block-number
              // 9563094
    async fn test_latest_common_height() -> Result<()> {
        let relayer = Relayer::read_only(
            "http://127.0.0.1:8545".parse()?,
            "0xaAD39528eB8b3c70b613C442F351610969974fDF".parse()?,
            EsploraClient::new(bitcoin::Network::Signet)?,
        );

        assert_eq!(relayer.relayed_height().await?, 238513);
        assert_eq!(relayer.latest_common_height().await?, 238512);
        Ok(())
    }

    #[tokio::test]
    async fn test_heaviest_relayed_block_header() -> Result<()> {
        let relayer = Relayer::read_only(
            "https://bob-sepolia.rpc.gobob.xyz/".parse()?,
            "0xaAD39528eB8b3c70b613C442F351610969974fDF".parse()?,
            EsploraClient::new(bitcoin::Network::Bitcoin)?,
        );

        // Not much we can easily test except that we find an actual block header
        relayer.get_heaviest_relayed_block_header().await?;

        Ok(())
    }
}
