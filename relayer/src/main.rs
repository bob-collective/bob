use alloy::{network::EthereumWallet, primitives::Address, signers::local::PrivateKeySigner};
use clap::Parser;
use eyre::Result;
use reqwest::Url;
use utils::EsploraClient;

use bob_relayer::Relayer;

/// Relayer
#[derive(Debug, Parser)]
#[command(name = "app")]
struct App {
    /// Ethereum RPC URL
    #[clap(long, env = "ETH_RPC_URL", default_value = "http://localhost:8545")]
    eth_rpc_url: String,

    /// The relayer's private key for submitting txs
    #[clap(long, env = "PRIVATE_KEY")]
    private_key: String,

    /// The relay address
    #[clap(long, env = "RELAY_ADDRESS")]
    relay_address: Address,

    /// The esplora URL
    #[clap(long, env = "ESPLORA_URL")]
    esplora_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init();
    let app = App::parse();

    println!("Starting relayer...");

    let privk = app.private_key.trim().strip_prefix("0x").expect("Requires private key");
    let signer: PrivateKeySigner = privk.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);
    let rpc_url: Url = app.eth_rpc_url.parse()?;
    let esplora_client = app
        .esplora_url
        .map(EsploraClient::new_with_url)
        .unwrap_or(EsploraClient::new(bitcoin::Network::Bitcoin))?;

    let relayer = Relayer::new(rpc_url, app.relay_address, esplora_client, wallet);
    relayer.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{network::EthereumWallet, node_bindings::Anvil, signers::local::PrivateKeySigner};

    #[tokio::test]
    async fn test() -> Result<()> {
        let anvil = Anvil::new().try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer.clone());
        let rpc_url = anvil.endpoint_url();

        let esplora_client = EsploraClient::new(bitcoin::Network::Bitcoin)?;

        let period_start_height = 201600;
        // change this to test different headers
        let genesis_height = period_start_height + 2014;

        let contract_address = Relayer::deploy_contract(
            &rpc_url,
            &wallet,
            &esplora_client,
            genesis_height,
            period_start_height,
        )
        .await?;

        // let block_headers = esplora_client
        //     .get_raw_block_header(&esplora_client.get_block_hash(genesis_height + 1).await?)
        //     .await?;

        // let receipt = contract
        //     .addHeaders(Bytes::from(genesis_block_header), Bytes::from(block_headers))
        //     .send()
        //     .await?
        //     .get_receipt()
        //     .await?;

        // assert!(receipt.status());

        Relayer::new(rpc_url, contract_address, esplora_client, wallet).run_once().await?;

        Ok(())
    }
}
