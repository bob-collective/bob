use bitcoin::{
    address::NetworkChecked,
    consensus::{self, serialize},
    hashes::{hex::FromHex, sha256d::Hash as Sha256dHash, Hash},
    hex::HexToBytesError,
    opcodes,
    script::{Error as ScriptError, Instruction},
    Address, Amount, MerkleBlock, Network, ScriptBuf, Txid,
};
use bitcoincore_rpc::{
    bitcoin::{BlockHash, Transaction},
    json,
    json::{EstimateMode, TestMempoolAcceptResult},
    jsonrpc::{error::RpcError, Error as JsonRpcError},
    Auth, Client, Error as BitcoinError, RpcApi,
};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::error::Category as SerdeJsonCategory;
use std::{sync::Arc, time::Duration};
use tokio::time::{error::Elapsed, sleep, timeout};
use tracing::*;

pub(crate) const RETRY_DURATION: Duration = Duration::from_secs(1);

// https://github.com/bitcoin/bitcoin/blob/be3af4f31089726267ce2dbdd6c9c153bb5aeae1/src/rpc/protocol.h#L43
#[derive(Debug, FromPrimitive, PartialEq, Eq)]
pub enum BitcoinRpcError {
    /// Standard JSON-RPC 2.0 errors
    RpcInvalidRequest = -32600,
    RpcMethodNotFound = -32601,
    RpcInvalidParams = -32602,
    RpcInternalError = -32603,
    RpcParseError = -32700,

    /// General application defined errors
    RpcMiscError = -1,
    RpcTypeError = -3,
    RpcInvalidAddressOrKey = -5,
    RpcOutOfMemory = -7,
    RpcInvalidParameter = -8,
    RpcDatabaseError = -20,
    RpcDeserializationErrr = -22,
    RpcVerifyError = -25,
    RpcVerifyRejected = -26,
    RpcVerifyAlreadyInChain = -27,
    RpcInWarmup = -28,
    RpcMethodDeprecated = -32,

    /// Aliases for backward compatibility
    // RpcTransactionError           = RpcVerifyError,
    // RpcTransactionRejected        = RpcVerifyRejected,
    // RpcTransactionAlreadyInChain  = RpcVerifyAlreadyInChain,

    /// P2P client errors
    RpcClientNotConnected = -9,
    RpcClientInInitialDownload = -10,
    RpcClientNodeAlreadyAdded = -23,
    RpcClientNodeNotAdded = -24,
    RpcClientNodeNotConnected = -29,
    RpcClientInvalidIpOrSubnet = -30,
    RpcClientP2PDisabled = -31,

    /// Chain errors
    RpcClientMempoolDisabled = -33,

    /// Wallet errors
    RpcWalletError = -4,
    RpcWalletInsufficientFunds = -6,
    RpcWalletInvalidLabelName = -11,
    RpcWalletKeypoolRanOut = -12,
    RpcWalletUnlockNeeded = -13,
    RpcWalletPassphraseIncorrect = -14,
    RpcWalletWrongEncState = -15,
    RpcWalletEncryptionFailed = -16,
    RpcWalletAlreadyUnlocked = -17,
    RpcWalletNotFound = -18,
    RpcWalletNotSpecified = -19,

    /// Backwards compatible aliases
    // RpcWalletInvalidAccountName = RpcWalletInvalidLabelName,

    /// Unused reserved codes.
    RpcForbiddenBySafeMode = -2,

    /// Unknown error code (not in spec).
    RpcUnknownError = 0,
}

/// A representation of a fee rate. Bitcoin Core uses different units in different
/// versions. To avoid burdening the user with using the correct unit, this struct
/// provides an umambiguous way to represent the fee rate, and the lib will perform
/// the necessary conversions.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct FeeRate(Amount);

impl FeeRate {
    /// Construct FeeRate from the amount per vbyte
    pub fn per_vbyte(amount_per_vbyte: Amount) -> Self {
        // internal representation is amount per vbyte
        Self(amount_per_vbyte)
    }

    /// Construct FeeRate from the amount per kilo-vbyte
    pub fn per_kvbyte(amount_per_kvbyte: Amount) -> Self {
        // internal representation is amount per vbyte, so divide by 1000
        Self::per_vbyte(amount_per_kvbyte / 1000)
    }

    pub fn to_sat_per_vbyte(&self) -> f64 {
        // multiply by the number of decimals to get sat
        self.0.to_sat() as f64 // TODO: Changed this
    }

    pub fn to_btc_per_kvbyte(&self) -> f64 {
        // divide by 10^8 to get btc/vbyte, then multiply by 10^3 to get btc/kbyte
        self.0.to_sat() as f64 / 100_000.0
    }

    /// Converts the internal fee rate (per vB) to BTC/kvB for RPC compatibility.
    pub fn to_btc_per_kvbyte_amount(&self) -> Amount {
        // 1000 vB in 1 kvB → multiply the per-vB amount by 1000
        Amount::from_sat(self.0.to_sat() * 1000)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BumpFeeOptions {
    /// Confirmation target in blocks.
    pub conf_target: Option<u16>,
    /// Specify a fee rate instead of relying on the built-in fee estimator.
    pub fee_rate: Option<FeeRate>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    pub replaceable: Option<bool>,
    /// The fee estimate mode
    pub estimate_mode: Option<EstimateMode>,
}

#[derive(Serialize, Clone, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializableBumpFeeOptions {
    #[serde(rename = "conf_target", skip_serializing_if = "Option::is_none")]
    /// Confirmation target in blocks.
    pub conf_target: Option<u16>,
    /// Specify a fee rate instead of relying on the built-in fee estimator.
    #[serde(rename = "fee_rate")]
    pub fee_rate: Option<f64>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaceable: Option<bool>,
    /// The fee estimate mode
    #[serde(rename = "estimate_mode", skip_serializing_if = "Option::is_none")]
    pub estimate_mode: Option<EstimateMode>,
}

impl BumpFeeOptions {
    pub fn to_serializable(&self, version: usize) -> SerializableBumpFeeOptions {
        let fee_rate = self.fee_rate.map(|x| {
            if version < 210000 {
                x.to_btc_per_kvbyte()
            } else {
                x.to_sat_per_vbyte()
            }
        });

        SerializableBumpFeeOptions {
            fee_rate,
            conf_target: self.conf_target,
            replaceable: self.replaceable,
            estimate_mode: self.estimate_mode,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BumpFeeResult {
    /// The base64-encoded unsigned PSBT of the new transaction. Only returned when wallet private
    /// keys are disabled.
    pub psbt: Option<String>,
    /// The id of the new transaction. Only returned when wallet private keys are enabled.
    pub txid: Option<bitcoin::Txid>,
    /// The fee of the original transaction (before bumping), denominated in BTC.
    pub origfee: f64,
    /// The fee of the newly created bumped transaction, denominated in BTC.
    pub fee: f64,
    /// Errors encountered during processing.
    pub errors: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PsbtBumpFeeResult {
    /// The base64-encoded unsigned PSBT of the new transaction. Only returned when wallet private
    /// keys are disabled.
    pub psbt: Option<String>,
    /// The fee of the original transaction (before bumping), denominated in BTC.
    pub origfee: f64,
    /// The fee of the newly created bumped transaction, denominated in BTC.
    pub fee: f64,
    /// Errors encountered during processing.
    pub errors: Vec<String>,
}

impl From<RpcError> for BitcoinRpcError {
    fn from(err: RpcError) -> Self {
        match num::FromPrimitive::from_i32(err.code) {
            Some(err) => err,
            None => Self::RpcUnknownError,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("BitcoinError: {0}")]
    BitcoinError(#[from] BitcoinError),
    #[error("ScriptError: {0}")]
    ScriptError(#[from] ScriptError),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("HexError: {0}")]
    HexError(#[from] HexToBytesError),
    #[error("EncodeError: {0}")]
    EncodeError(#[from] bitcoin::consensus::encode::Error),
    #[error("Missing Txid")]
    MissingTxId,
    #[error("Timeout: {0}")]
    TimeElapsed(#[from] Elapsed),
    #[error("Missing OP_RETURN")]
    MissingOpReturn,
    #[error("Invalid bitcoin tx")]
    InvalidBitcoinTx,
    #[error("Invalid network")]
    InvalidNetwork,
    #[error("Invalid recipient")]
    InvalidRecipient,
    #[error("Hex decoding error: {0}")]
    HexDecodeError(#[from] hex::FromHexError),
    #[error("Finalized PSBT did not return a raw transaction hex")]
    MissingRawTxHex,
}

#[derive(Clone)]
pub struct BitcoinClient {
    pub rpc: Arc<Client>,
}

impl From<Client> for BitcoinClient {
    fn from(client: Client) -> Self {
        Self { rpc: Arc::new(client) }
    }
}

impl BitcoinClient {
    pub fn new(url: &str, rpc_user: impl Into<String>, rpc_pass: impl Into<String>) -> Self {
        Self {
            rpc: Client::new(url, Auth::UserPass(rpc_user.into(), rpc_pass.into())).unwrap().into(),
        }
    }

    pub async fn connect(&self, connection_timeout: Duration) -> Result<(), Error> {
        info!("Connecting to bitcoin...");
        timeout(connection_timeout, async move {
            loop {
                match self.rpc.get_blockchain_info() {
                    Err(BitcoinError::JsonRpc(JsonRpcError::Transport(_))) =>
                    {
                        trace!("A transport error occurred while attempting to communicate with bitcoin. Typically this indicates a failure to connect");
                        sleep(RETRY_DURATION).await;
                        continue;
                    }
                    Err(BitcoinError::JsonRpc(JsonRpcError::Rpc(err)))
                        if BitcoinRpcError::from(err.clone()) == BitcoinRpcError::RpcInWarmup =>
                    {
                        // may be loading block index or verifying wallet
                        trace!("Bitcoin still in warm up");
                        sleep(RETRY_DURATION).await;
                        continue;
                    }
                    Err(BitcoinError::JsonRpc(JsonRpcError::Json(err))) if err.classify() == SerdeJsonCategory::Syntax => {
                        // invalid response, can happen if server is in shutdown
                        trace!("Bitcoin gave an invalid response: {}", err);
                        sleep(RETRY_DURATION).await;
                        continue;
                    }
                    Ok(info) => {
                        info!("Connected to {}", info.chain);
                        return Ok(());
                    }
                    Err(err) => return Err(err),
                }
            }
        })
        .await?.map_err(Error::BitcoinError)
    }

    pub fn network(&self) -> Result<Network, Error> {
        let info = self.rpc.get_blockchain_info()?;
        Ok(info.chain)
    }

    pub fn get_block_hash(&self, height: u64) -> Result<BlockHash, Error> {
        Ok(self.rpc.get_block_hash(height)?)
    }

    pub fn get_raw_block_header(&self, hash: &BlockHash) -> Result<Vec<u8>, Error> {
        Ok(serialize(&self.rpc.get_block_header(hash)?))
    }

    pub fn get_raw_tx(
        &self,
        txid: &Txid,
        block_hash: Option<&BlockHash>,
    ) -> Result<Vec<u8>, Error> {
        Ok(Vec::<u8>::from_hex(&self.rpc.get_raw_transaction_hex(txid, block_hash)?)?)
    }

    pub fn get_tx(
        &self,
        txid: &Txid,
        block_hash: Option<&BlockHash>,
    ) -> Result<Transaction, Error> {
        Ok(consensus::deserialize(&self.get_raw_tx(txid, block_hash)?)?)
    }

    pub fn get_merkle_block(
        &self,
        txid: Txid,
        block_hash: &BlockHash,
    ) -> Result<MerkleBlock, Error> {
        let block = self.rpc.get_block(block_hash)?;
        Ok(MerkleBlock::from_block_with_predicate(&block, |t| txid.eq(t)))
    }

    pub fn get_tx_merkle_proof(
        &self,
        tx_hash: &Txid,
        block_hash: &BlockHash,
    ) -> Result<(Vec<Sha256dHash>, usize), Error> {
        let block = self.rpc.get_block(block_hash)?;
        let txids: Vec<_> = block.txdata.iter().map(|tx| tx.compute_txid()).collect();
        let pos = txids.iter().position(|txid| txid == tx_hash).ok_or(Error::MissingTxId)?;
        let txids = txids.into_iter().map(Sha256dHash::from).collect();

        let (branch, _root) = create_merkle_branch_and_root(txids, pos);
        Ok((branch, pos))
    }

    pub fn validate_and_send_raw_transaction(&self, tx: &Transaction) -> Result<Txid, Error> {
        match &self.rpc.test_mempool_accept(&[tx])?[..] {
            [TestMempoolAcceptResult { allowed: true, .. }] => {
                info!("Sending transaction");
                Ok(self.rpc.send_raw_transaction(tx)?)
            }
            [TestMempoolAcceptResult { txid, reject_reason: Some(reason), .. }]
                if matches!(reason.as_str(), "txn-already-known" | "txn-already-in-mempool") =>
            {
                Ok(*txid)
            }
            [TestMempoolAcceptResult { reject_reason: Some(reason), .. }] => {
                error!("Could not validate tx: {reason}");
                Err(Error::InvalidBitcoinTx)
            }
            _ => Err(Error::InvalidBitcoinTx),
        }
    }

    pub fn send_to_address_with_op_return(
        &self,
        address: Option<&Address<NetworkChecked>>,
        amount: Option<Amount>,
        op_return_data: Option<&Vec<u8>>,
        replaceable: Option<bool>,
        confirmation_target: Option<u32>,
        estimate_mode: Option<bitcoincore_rpc::json::EstimateMode>,
        fee_rate: Option<FeeRate>,
    ) -> Result<Txid, Error> {
        let tx = self.create_and_sign_tx(
            address,
            amount,
            op_return_data,
            replaceable,
            confirmation_target,
            estimate_mode,
            fee_rate,
        )?;
        let txid = self.validate_and_send_raw_transaction(&tx)?;
        Ok(txid)
    }

    pub fn create_and_sign_tx(
        &self,
        address: Option<&Address<NetworkChecked>>,
        amount: Option<Amount>,
        op_return_data: Option<&Vec<u8>>,
        replaceable: Option<bool>,
        confirmation_target: Option<u32>,
        estimate_mode: Option<bitcoincore_rpc::json::EstimateMode>,
        fee_rate: Option<FeeRate>,
    ) -> Result<Transaction, Error> {
        match (address, amount, op_return_data) {
            // txs must have at least one output
            (Some(_), None, _) | (None, Some(_), _) | (None, None, None) => {
                return Err(Error::InvalidRecipient);
            }
            _ => {}
        }

        let mut outputs = serde_json::Map::new();
        if let (Some(addr), Some(amt)) = (address, amount) {
            outputs.insert(addr.to_string(), serde_json::Value::from(amt.to_btc()));
        }
        if let Some(data) = op_return_data {
            outputs.insert("data".to_string(), serde_json::Value::String(hex::encode(data)));
        }
        let tx = self.rpc.call::<String>(
            "createrawtransaction",
            &[
                serde_json::to_value::<&[bitcoincore_rpc::json::CreateRawTransactionInput]>(&[])?,
                serde_json::to_value(outputs)?,
            ],
        )?;
        let tx = self
            .rpc
            .fund_raw_transaction(
                tx,
                Some(&bitcoincore_rpc::json::FundRawTransactionOptions {
                    add_inputs: None,
                    change_address: None,
                    change_position: None,
                    change_type: None,
                    include_watching: None,
                    lock_unspents: None,
                    fee_rate: fee_rate.map(|fr| fr.to_btc_per_kvbyte_amount()), // `fee_rate` must be in BTC/kvB as expected by `fundrawtransaction`,
                    subtract_fee_from_outputs: None,
                    replaceable,
                    conf_target: confirmation_target,
                    estimate_mode,
                }),
                None,
            )?
            .hex;
        let signed_tx = self.rpc.sign_raw_transaction_with_wallet(&tx, None, None)?;
        let transaction = signed_tx.transaction()?;
        Ok(transaction)
    }

    pub fn bump_fee(
        &self,
        txid: &Txid,
        options: Option<&BumpFeeOptions>,
    ) -> Result<BumpFeeResult, Error> {
        // Serialize options if provided
        let opts = match options {
            Some(options) => Some(options.to_serializable(self.rpc.version()?)),
            None => None,
        };

        // Prepare arguments
        let args = vec![serde_json::to_value(txid)?, serde_json::to_value(opts)?];

        // Call the "bumpfee" RPC method with borrowed args
        let result = self.rpc.call("bumpfee", &args);

        // Handle the result
        match result {
            Ok(result_value) => {
                // Try to deserialize the result into the expected BumpFeeResult
                let result: Result<BumpFeeResult, _> = serde_json::from_value(result_value);

                match result {
                    Ok(bump_fee_result) => Ok(bump_fee_result),
                    Err(err) => {
                        println!("Failed to deserialize into BumpFeeResult");
                        Err(err.into())
                    }
                }
            }
            Err(err) => Err(err.into()), // Handle the case where the RPC call fails
        }
    }

    pub fn psbt_bump_fee(
        &self,
        txid: &Txid,
        options: Option<&BumpFeeOptions>,
    ) -> Result<PsbtBumpFeeResult, Error> {
        // Serialize options if provided
        let opts = match options {
            Some(options) => Some(options.to_serializable(self.rpc.version()?)),
            None => None,
        };

        // Prepare arguments
        let args = vec![serde_json::to_value(txid)?, serde_json::to_value(opts)?];

        // Call the "psbtbumpfee" RPC method
        let result = self.rpc.call("psbtbumpfee", &args);

        // Handle the result
        match result {
            Ok(result_value) => {
                let result: Result<PsbtBumpFeeResult, _> = serde_json::from_value(result_value);
                match result {
                    Ok(bump_fee_result) => Ok(bump_fee_result),
                    Err(err) => {
                        println!("Failed to deserialize into PsbtBumpFeeResult");
                        Err(err.into())
                    }
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    pub fn sign_and_finalize_psbt(
        &self,
        psbt: &str,
        sign: Option<bool>,
        sighash_type: Option<json::SigHashType>,
        bip32derivs: Option<bool>,
    ) -> Result<Transaction, Error> {
        let wallet_process_psbt =
            self.rpc.wallet_process_psbt(psbt, sign, sighash_type, bip32derivs)?;
        let finalized = self.rpc.finalize_psbt(&wallet_process_psbt.psbt, Some(true))?;
        let tx_bytes = finalized.hex.ok_or(Error::MissingRawTxHex)?;
        let tx: Transaction = consensus::deserialize(&tx_bytes)?;
        Ok(tx)
    }
}

fn merklize(left: Sha256dHash, right: Sha256dHash) -> Sha256dHash {
    let data = [&left[..], &right[..]].concat();
    Sha256dHash::hash(&data)
}

// See: https://github.com/Blockstream/electrs/blob/4ff496a9cbce7259ab885a7f72c35aff7135384f/src/util/electrum_merkle.rs#L86
fn create_merkle_branch_and_root(
    mut hashes: Vec<Sha256dHash>,
    mut index: usize,
) -> (Vec<Sha256dHash>, Sha256dHash) {
    let mut merkle = vec![];
    while hashes.len() > 1 {
        if !hashes.len().is_multiple_of(2) {
            let last = *hashes.last().unwrap();
            hashes.push(last);
        }
        index = if index.is_multiple_of(2) { index + 1 } else { index - 1 };
        merkle.push(hashes[index]);
        index /= 2;
        hashes = hashes.chunks(2).map(|pair| merklize(pair[0], pair[1])).collect()
    }
    (merkle, hashes[0])
}

pub fn extract_op_return_data(output_script: &ScriptBuf) -> Result<Vec<u8>, Error> {
    let instructions = output_script.instructions().collect::<Result<Vec<Instruction>, _>>()?;
    match instructions[..] {
        [Instruction::Op(opcodes::all::OP_RETURN), Instruction::PushBytes(data)] => {
            Ok(data.as_bytes().to_vec())
        }
        _ => Err(Error::MissingOpReturn),
    }
}
