mod bitcoin_client;
mod bitcoin_core;
mod esplora_client;

pub use bitcoin_client::*;
pub use bitcoin_core::*;
pub use esplora_client::*;

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::Amount;
    use bitcoincore_rpc::RpcApi;
    use eyre::Result;

    #[tokio::test]
    async fn test_validate() -> Result<()> {
        let bitcoin = BitcoinCore::new().spawn();
        bitcoin.fund_wallet("Alice")?;
        let alice = BitcoinClient::from(bitcoin.client(Some("Alice"))?);
        let address = alice.rpc.get_new_address(None, None)?.assume_checked();

        // 1) in mempool
        let txid = alice.rpc.send_to_address(
            &address,
            Amount::from_sat(1000000),
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        let tx = alice.rpc.get_raw_transaction(&txid, None)?;
        alice.validate_and_send_raw_transaction(&tx)?;

        // 2) with confirmations
        let txid = alice.rpc.send_to_address(
            &address,
            Amount::from_sat(1000000),
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        let tx = alice.rpc.get_raw_transaction(&txid, None)?;
        alice
            .rpc
            .generate_to_address(6, &alice.rpc.get_new_address(None, None)?.assume_checked())?;
        alice.validate_and_send_raw_transaction(&tx)?;

        // 3) not in mempool
        let tx = alice.rpc.call::<String>(
            "createrawtransaction",
            &[
                serde_json::to_value::<&[bitcoincore_rpc::json::CreateRawTransactionInput]>(&[])?,
                serde_json::to_value(serde_json::Map::from_iter(vec![(
                    address.to_string(),
                    serde_json::Value::from(Amount::from_sat(100000).to_btc()),
                )]))?,
            ],
        )?;
        let tx = alice.rpc.fund_raw_transaction(tx, None, None)?.hex;
        let tx = alice.rpc.sign_raw_transaction_with_wallet(&tx, None, None)?.hex;
        alice.validate_and_send_raw_transaction(&bitcoin::consensus::deserialize(&tx).unwrap())?;

        Ok(())
    }
}
