use bitcoin::{
    block::Header, consensus, hashes::hex::FromHex, BlockHash, Network, Transaction, Txid,
};
use eyre::Result;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::str::FromStr;
use tracing::*;

const ESPLORA_MAINNET_URL: &str = "https://blockstream.info/api/";
const ESPLORA_TESTNET_URL: &str = "https://blockstream.info/testnet/api/";
const ESPLORA_LOCALHOST_URL: &str = "http://localhost:3002";

#[derive(Debug, Deserialize)]
pub struct MerkleProof {
    pub block_height: u32,
    pub merkle: Vec<String>,
    pub pos: u32,
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

#[derive(Clone)]
pub struct EsploraClient {
    url: Url,
    cli: Client,
}

impl EsploraClient {
    pub fn new(esplora_url: Option<String>, network: Network) -> Result<Self> {
        Ok(Self {
            url: esplora_url
                .unwrap_or_else(|| {
                    match network {
                        Network::Bitcoin => ESPLORA_MAINNET_URL,
                        Network::Testnet => ESPLORA_TESTNET_URL,
                        _ => ESPLORA_LOCALHOST_URL,
                    }
                    .to_owned()
                })
                .parse()?,
            cli: Client::new(),
        })
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

    pub async fn get_tx_hex(&self, txid: &Txid) -> Result<String> {
        self.get(&format!("/tx/{txid}/hex")).await
    }

    pub async fn get_raw_tx(&self, txid: &Txid) -> Result<Vec<u8>> {
        Ok(Vec::<u8>::from_hex(&self.get_tx_hex(txid).await?)?)
    }

    pub async fn get_merkle_proof(&self, txid: &Txid) -> Result<MerkleProof> {
        self.get_and_decode(&format!("/tx/{txid}/merkle-proof")).await
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

    pub async fn get_raw_block_header_at_height(&self, height: u32) -> Result<Vec<u8>> {
        self.get_raw_block_header(&self.get_block_hash(height).await?).await
    }

    pub async fn get_block_header_at_height(&self, height: u32) -> Result<Header> {
        self.get_block_header(&self.get_block_hash(height).await?).await
    }

    pub async fn get_chain_height(&self) -> Result<u32> {
        Ok(self.get("blocks/tip/height").await?.parse()?)
    }

    pub async fn get_tx_status(&self, txid: &Txid) -> Result<TransactionStatus> {
        self.get_and_decode(&format!("/tx/{txid}/status")).await
    }

    pub async fn send_transaction(&self, tx: &Transaction) -> Result<Txid> {
        let url = self.url.join("/tx")?;
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
        let txid = tx.txid();
        let tx_status = self.get_tx_status(&txid).await?;
        match tx_status {
            TransactionStatus { confirmed: true, block_height: Some(_), .. } => Ok(txid),
            _ => {
                info!("Sending transaction");
                self.send_transaction(tx).await
            }
        }
    }
}
