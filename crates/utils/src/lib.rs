mod bitcoin_client;
mod bitcoin_core;
mod esplora;
mod esplora_client;
mod mempool_client;

pub use bitcoin_client::*;
pub use bitcoin_core::*;
pub use esplora::*;
pub use esplora_client::*;
pub use mempool_client::*;

pub(crate) fn unused_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to create TCP listener to find unused port");

    let local_addr =
        listener.local_addr().expect("Failed to read TCP listener local_addr to find unused port");
    local_addr.port()
}

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

    #[tokio::test]
    async fn test_op_return() -> Result<()> {
        let bitcoin = BitcoinCore::new().spawn();
        bitcoin.fund_wallet("Alice")?;
        let alice = BitcoinClient::from(bitcoin.client(Some("Alice"))?);
        let address = alice.rpc.get_new_address(None, None)?.assume_checked();

        assert!(matches!(
            alice.send_to_address_with_op_return(None, None, None, None, None, None),
            Err(bitcoin_client::Error::InvalidRecipient)
        ));

        assert!(matches!(
            alice.send_to_address_with_op_return(Some(&address), None, None, None, None, None),
            Err(bitcoin_client::Error::InvalidRecipient)
        ));

        let op_return_data = vec![1; 32];
        let txid = alice.send_to_address_with_op_return(
            Some(&address),
            Some(Amount::from_btc(0.1).expect("Invalid BTC amount")),
            Some(&op_return_data),
            None,
            None,
            None,
        )?;

        let tx = alice.get_tx(&txid, None)?;

        let mut actual_data = None;
        for out in tx.output {
            if let Ok(data) = extract_op_return_data(&out.script_pubkey) {
                actual_data = Some(data);
            }
        }

        assert_eq!(op_return_data, actual_data.expect("Should have OP_RETURN"));

        Ok(())
    }
}
