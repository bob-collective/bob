use eyre::{bail, Result};
use reqwest::{Client, Url};
use serde::Deserialize;
use std::str::FromStr;

const MEMPOOL_MAINNET_URL: &str = "https://mempool.space/api/v1/";
const MEMPOOL_TESTNET_URL: &str = "https://mempool.space/testnet/api/v1/";
const MEMPOOL_SIGNET_URL: &str = "https://mempool.space/signet/api/v1/";
const MEMPOOL_TESTNET4_URL: &str = "https://mempool.space/testnet4/api/v1/";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimates {
    pub fastest_fee: u32,
    pub half_hour_fee: u32,
    pub hour_fee: u32,
    pub economy_fee: u32,
    pub minimum_fee: u32,
}

#[derive(Clone)]
pub struct MempoolClient {
    url: Url,
    cli: Client,
}

impl MempoolClient {
    pub fn new(mempool_url: Option<String>, network: bitcoin::Network) -> Result<Self> {
        let url = match mempool_url {
            Some(url) => url,
            None => match network {
                bitcoin::Network::Bitcoin => MEMPOOL_MAINNET_URL.to_owned(),
                bitcoin::Network::Testnet => MEMPOOL_TESTNET_URL.to_owned(),
                bitcoin::Network::Signet => MEMPOOL_SIGNET_URL.to_owned(),
                bitcoin::Network::Testnet4 => MEMPOOL_TESTNET4_URL.to_owned(),
                _ => bail!("Unsupported Bitcoin network by Mempool: {:?}", network),
            },
        };

        Ok(Self { url: Url::from_str(&url)?, cli: Client::new() })
    }

    async fn get(&self, path: &str) -> Result<String> {
        let url = self.url.join(path)?;
        Ok(self.cli.get(url).send().await?.error_for_status()?.text().await?)
    }

    pub async fn get_fee_estimates(&self) -> Result<FeeEstimates> {
        let response = self.get("fees/recommended").await?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mempool_fees() -> Result<()> {
        let client = MempoolClient::new(None, bitcoin::Network::Signet)?;
        let fees = client.get_fee_estimates().await?;
        assert!(fees.fastest_fee > 0);
        assert!(fees.half_hour_fee > 0);
        assert!(fees.hour_fee > 0);
        assert!(fees.economy_fee > 0);
        assert!(fees.minimum_fee > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_mempool_client_for_regtest() -> Result<()> {
        let client = MempoolClient::new(None, bitcoin::Network::Regtest);
        assert!(client.is_err());
        Ok(())
    }
}
