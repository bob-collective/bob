use bitcoin::{
    block::Header, consensus, hashes::hex::FromHex, BlockHash, CompactTarget, MerkleBlock, Network,
    Transaction, TxMerkleNode, Txid,
};
use eyre::{Error, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::str::FromStr;
use tracing::*;

const ESPLORA_TESTNET_URL: &str = "https://btc-testnet.interlay.io";
const ESPLORA_MAINNET_URL: &str = "https://btc-mainnet.interlay.io";
const ESPLORA_LOCALHOST_URL: &str = "http://localhost:3002";
const ESPLORA_SIGNET_URL: &str = "https://btc-signet.gobob.xyz";

#[derive(Deserialize, Debug)]
pub struct Tx {
    pub txid: Txid,
    pub version: i32,
    pub locktime: u32,
    pub vin: Vec<VinFormat>,
    pub vout: Vec<VoutFormat>,
    /// Transaction size in raw bytes (NOT virtual bytes).
    pub size: usize,
    /// Transaction weight units.
    pub weight: u64,
    pub status: TransactionStatus,
    pub fee: u64,
}

// https://github.com/Blockstream/electrs/blob/adedee15f1fe460398a7045b292604df2161adc0/src/util/transaction.rs#L17-L26
#[derive(Debug, Deserialize)]
pub struct TransactionStatus {
    pub confirmed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MerkleProof {
    pub block_height: u32,
    pub merkle: Vec<String>,
    pub pos: u32,
}

#[derive(Deserialize, Debug)]
pub struct VoutFormat {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u32,
}

#[derive(Deserialize, Debug)]
pub struct VinFormat {
    pub txid: String,
    pub vout: u32,
    pub is_coinbase: bool,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub sequence: u32,
    pub prevout: Option<VoutFormat>,
}

#[derive(Deserialize, Debug)]
pub struct TransactionFormat {
    pub txid: String,
    pub version: u32,
    pub locktime: u32,
    pub size: u32,
    pub weight: u32,
    pub fee: u32,
    pub vin: Vec<VinFormat>,
    pub vout: Vec<VoutFormat>,
    pub status: TransactionStatus,
}

impl MerkleProof {
    pub fn encode(self) -> Vec<u8> {
        // convert to little-endian
        self.merkle
            .into_iter()
            .flat_map(|item| {
                let mut data = Vec::<u8>::from_hex(&item).unwrap();
                data.reverse();
                data
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct BlockValue {
    pub id: BlockHash,
    pub height: u32,
    pub version: u32,
    pub timestamp: u32,
    pub tx_count: u32,
    pub size: u32,
    pub weight: u64,
    pub merkle_root: TxMerkleNode,
    pub previousblockhash: Option<BlockHash>,
    pub mediantime: u32,
    pub nonce: u32,
    pub bits: CompactTarget,
    pub difficulty: f64,
}

#[derive(Clone)]
pub struct EsploraClient {
    url: Url,
    cli: Client,
}

impl EsploraClient {
    pub fn new(network: Network) -> Result<Self> {
        Self::new_with_url(
            match network {
                Network::Bitcoin => ESPLORA_MAINNET_URL,
                Network::Testnet => ESPLORA_TESTNET_URL,
                Network::Signet => ESPLORA_SIGNET_URL,
                _ => ESPLORA_LOCALHOST_URL,
            }
            .to_owned(),
        )
    }

    pub fn new_with_url(esplora_url: String) -> Result<Self> {
        Ok(Self { url: esplora_url.parse()?, cli: Client::new() })
    }

    async fn get(&self, path: &str) -> Result<String> {
        let url = self.url.join(path)?;
        Ok(self.cli.get(url).send().await?.error_for_status()?.text().await?)
    }

    // only use this for parsing valid json, it will fail on strings
    async fn get_and_decode<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let body = self.get(path).await?;
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_tx(&self, txid: &Txid) -> Result<Transaction> {
        Ok(consensus::deserialize(&self.get_raw_tx(txid).await?)?)
    }

    pub async fn get_tx_hex(&self, txid: &Txid) -> Result<String> {
        self.get(&format!("/tx/{txid}/hex")).await
    }

    pub async fn get_tx_info(&self, txid: &Txid) -> Result<Tx, Error> {
        let path = format!("/tx/{txid}");
        let tx = self.get_and_decode::<Tx>(&path).await?;
        Ok(tx)
    }

    pub async fn get_raw_tx(&self, txid: &Txid) -> Result<Vec<u8>> {
        Ok(Vec::<u8>::from_hex(&self.get_tx_hex(txid).await?)?)
    }

    pub async fn get_merkleblock_proof(&self, txid: &Txid) -> Result<MerkleBlock> {
        Ok(consensus::deserialize(&Vec::<u8>::from_hex(
            &self.get(&format!("tx/{txid}/merkleblock-proof")).await?,
        )?)?)
    }

    pub async fn get_merkle_proof(&self, txid: &Txid) -> Result<MerkleProof> {
        self.get_and_decode(&format!("tx/{txid}/merkle-proof")).await
    }

    pub async fn get_transactions_by_scripthash(
        &self,
        scripthash: &str,
    ) -> Result<Vec<TransactionFormat>> {
        self.get_and_decode(&format!("scripthash/{scripthash}/txs")).await
    }

    pub async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        let response = self.get(&format!("block-height/{height}")).await?;
        Ok(BlockHash::from_str(&response)?)
    }

    pub async fn get_raw_block_header(&self, hash: &BlockHash) -> Result<Vec<u8>> {
        Ok(Vec::<u8>::from_hex(&self.get(&format!("block/{hash}/header")).await?)?)
    }

    pub async fn get_block_header(&self, hash: &BlockHash) -> Result<Header> {
        Ok(consensus::deserialize(&self.get_raw_block_header(hash).await?)?)
    }

    pub async fn get_block_value(&self, hash: &BlockHash) -> Result<BlockValue> {
        self.get_and_decode(&format!("block/{hash}")).await
    }

    pub async fn get_raw_block_header_at_height(&self, height: u32) -> Result<Vec<u8>> {
        self.get_raw_block_header(&self.get_block_hash(height).await?).await
    }

    pub async fn get_block_header_at_height(&self, height: u32) -> Result<Header> {
        self.get_block_header(&self.get_block_hash(height).await?).await
    }

    pub async fn get_block_txids(&self, hash: &BlockHash) -> Result<Vec<Txid>> {
        self.get_and_decode(&format!("block/{hash}/txids")).await
    }

    pub async fn get_chain_height(&self) -> Result<u32> {
        Ok(self.get("blocks/tip/height").await?.parse()?)
    }

    pub async fn get_tx_status(&self, txid: &Txid) -> Result<TransactionStatus> {
        self.get_and_decode(&format!("tx/{txid}/status")).await
    }

    /// Fetch the transaction at the specified index within the given block.
    pub async fn get_block_txid_by_index(
        &self,
        block_hash: &BlockHash,
        index: usize,
    ) -> Result<Txid> {
        // 1. Get the txid at the index in the block
        let path = format!("block/{block_hash}/txid/{index}");
        let txid_str = self.get(&path).await?;
        let txid = Txid::from_str(&txid_str)?;
        Ok(txid)
    }

    pub async fn send_transaction(&self, tx: &Transaction) -> Result<Txid> {
        let url = self.url.join("tx")?;
        let txid = self
            .cli
            .post(url)
            .body(hex::encode(bitcoin::consensus::serialize(tx)))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        Ok(Txid::from_str(&txid)?)
    }

    pub async fn validate_and_send_raw_transaction(&self, tx: &Transaction) -> Result<Txid> {
        let txid = tx.compute_txid();
        let tx_status = self.get_tx_status(&txid).await?;
        match tx_status {
            TransactionStatus { confirmed: true, block_height: Some(_), .. } => Ok(txid),
            _ => {
                info!("Sending transaction");
                self.send_transaction(tx).await
            }
        }
    }

    pub async fn get_bitcoin_network(&self) -> Result<Network> {
        let url_str = self.url.as_str();

        match url_str {
            _ if url_str.contains(ESPLORA_MAINNET_URL) => Ok(Network::Bitcoin),
            _ if url_str.contains(ESPLORA_TESTNET_URL) => Ok(Network::Testnet),
            _ if url_str.contains(ESPLORA_LOCALHOST_URL) => Ok(Network::Regtest),
            _ if url_str.contains(ESPLORA_SIGNET_URL) => Ok(Network::Signet),
            _ => Err(Error::msg("Unknown network for URL: {url_str}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // construct the MerkleBlock from the header and txs
    async fn build_merkle_block(
        esplora_client: &EsploraClient,
        txid: &Txid,
        block_hash: &BlockHash,
    ) -> Result<MerkleBlock> {
        let header = esplora_client.get_block_header(block_hash).await?;
        let txids = esplora_client.get_block_txids(block_hash).await?;
        Ok(MerkleBlock::from_header_txids_with_predicate(&header, &txids, |t| txid.eq(t)))
    }

    #[tokio::test]
    async fn test_esplora() -> Result<()> {
        let esplora_client = EsploraClient::new(Network::Bitcoin)?;
        let txid =
            Txid::from_str("aaddbc39689a3d63b3bcaafc6d1440ef911ac30bc0fe4679b891bf3e389fb053")?;
        let left = esplora_client.get_merkleblock_proof(&txid).await?;

        let mut matches: Vec<Txid> = vec![];
        let mut index: Vec<u32> = vec![];
        left.extract_matches(&mut matches, &mut index)?;
        matches.first().filter(|x| txid == **x).unwrap();

        let right = build_merkle_block(
            &esplora_client,
            &txid,
            &BlockHash::from_str(
                "00000000000000000000b1c1dc40e2299515217ff11745e07a1f9078f06cb783",
            )?,
        )
        .await?;

        // for some reason left has more bits so just compare the header
        assert_eq!(left.header, right.header);

        Ok(())
    }

    #[tokio::test]
    async fn test_esplora_get_block_value() -> Result<()> {
        let esplora_client = EsploraClient::new(Network::Bitcoin)?;

        let block_hash = BlockHash::from_str(
            "00000000000000000000e4726002778d999b973fe138208ed5f6c23df0af7898",
        )?;

        assert_eq!(esplora_client.get_block_value(&block_hash).await.unwrap().height, 884457);
        Ok(())
    }

    #[tokio::test]
    async fn test_esplora_get_tx_info() -> Result<()> {
        let esplora_client = EsploraClient::new(Network::Bitcoin)?;

        let txid =
            Txid::from_str("7d572189e512cf1660abe079adde55ab38c61a714c622121756d062ee4934416")?;
        let tx = esplora_client.get_tx_info(&txid).await?;

        assert!(tx.status.confirmed);
        Ok(())
    }

    #[tokio::test]
    async fn test_esplora_fetch_coinbase_tx() -> Result<()> {
        let esplora_client = EsploraClient::new(Network::Bitcoin)?;

        let block_hash = BlockHash::from_str(
            "000000000000000000007baa43da74c78fbb7dc6b4bf5f1ef16d800a3b884051",
        )?;
        let coinbase_tx = esplora_client.get_block_txid_by_index(&block_hash, 0).await?;

        assert_eq!(
            coinbase_tx,
            Txid::from_str("33b36c41948b549159a200e9fed5027ef9a3fd5737a6bfe91094a61c8124c722")?
        );
        Ok(())
    }
}
