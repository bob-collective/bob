use crate::unused_port;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use eyre::Result;
use std::{
    ffi::OsString,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command},
    time::{Duration, Instant},
};
use tempfile::TempDir;

/// How long we will wait for bitcoin to indicate that it is ready.
const BITCOIN_STARTUP_TIMEOUT_MILLIS: u64 = 10_000;

pub(crate) const BITCOIN_RPC_USER: &str = "rpcuser";
pub(crate) const BITCOIN_RPC_PASSWORD: &str = "rpcpassword";

/// An bitcoind CLI instance. Will close the instance when dropped.
///
/// Construct this using [`BitcoinCore`].
pub struct BitcoinCoreInstance {
    pid: Child,
    _datadir: TempDir,
    rpcport: u16,
}

impl BitcoinCoreInstance {
    /// Returns the rpcport of this instance
    pub fn rpcport(&self) -> u16 {
        self.rpcport
    }

    /// Returns the JSON-RPC endpoint of this instance
    pub fn endpoint(&self) -> String {
        format!("http://localhost:{}", self.rpcport)
    }

    /// Returns the JSON-RPC wallet endpoint of this instance
    pub fn wallet_endpoint(&self, name: &str) -> String {
        format!("{}/wallet/{name}", self.endpoint())
    }

    /// Returns a client using the configured auth / wallet
    pub fn client(&self, wallet: Option<&str>) -> Result<Client> {
        Ok(Client::new(
            &wallet.map(|name| self.wallet_endpoint(name)).unwrap_or(self.endpoint()),
            Auth::UserPass(BITCOIN_RPC_USER.to_string(), BITCOIN_RPC_PASSWORD.to_string()),
        )?)
    }

    /// Helper to automatically create and fund wallet
    pub fn fund_wallet(&self, name: &str) -> Result<()> {
        let client = self.client(Some(name))?;
        client.create_wallet(name, None, None, None, None)?;
        let address = client.get_new_address(None, None)?.assume_checked();
        client.generate_to_address(101, &address)?;
        Ok(())
    }

    pub fn fund_to_specific_address(&self, address: &bitcoin::Address) -> Result<()> {
        let client = self.client(None)?;
        client.generate_to_address(101, address)?;
        Ok(())
    }
}

impl Drop for BitcoinCoreInstance {
    fn drop(&mut self) {
        self.pid.kill().expect("could not kill bitcoin");
    }
}

/// Builder for launching `bitcoind`.
///
/// # Panics
///
/// If `spawn` is called without `bitcoind` being available in the user's $PATH
///
/// # Example
///
/// ```no_run
/// use bob_utils::BitcoinCore;
///
/// let port = 8545u16;
/// let url = format!("http://localhost:{}", port).to_string();
///
/// let bitcoin = BitcoinCore::new()
///     .rpcport(port)
///     .spawn();
///
/// drop(bitcoin); // this will kill the instance
/// ```
#[derive(Debug, Clone, Default)]
#[must_use = "This Builder struct does nothing unless it is `spawn`ed"]
pub struct BitcoinCore {
    program: Option<PathBuf>,
    rpcport: Option<u16>,
    timeout: Option<u64>,
    args: Vec<OsString>,
}

impl BitcoinCore {
    /// Creates an empty BitcoinCore builder.
    /// The default port is 8545. The mnemonic is chosen randomly.
    ///
    /// # Example
    ///
    /// ```
    /// # use bob_utils::BitcoinCore;
    /// fn a() {
    ///  let bitcoin = BitcoinCore::default().spawn();
    ///
    ///  println!("Bitcoin running at `{}`", bitcoin.endpoint());
    /// # }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a BitcoinCore builder which will execute `bitcoind` at the given path.
    ///
    /// # Example
    ///
    /// ```
    /// # use bob_utils::BitcoinCore;
    /// fn a() {
    ///  let bitcoin = BitcoinCore::at("/usr/bin/bitcoind").spawn();
    ///
    ///  println!("Bitcoin running at `{}`", bitcoin.endpoint());
    /// # }
    /// ```
    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self::new().path(path)
    }

    /// Sets the `path` to the `bitcoind` cli
    ///
    /// By default, it's expected that `bitcoind` is in `$PATH`, see also
    /// [`std::process::Command::new()`]
    pub fn path<T: Into<PathBuf>>(mut self, path: T) -> Self {
        self.program = Some(path.into());
        self
    }

    /// Sets the JSON-RPC port which will be used when the `bitcoind` instance is launched.
    pub fn rpcport<T: Into<u16>>(mut self, port: T) -> Self {
        self.rpcport = Some(port.into());
        self
    }

    /// Sets the timeout which will be used when the `bitcoind` instance is launched.
    pub fn timeout<T: Into<u64>>(mut self, timeout: T) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    pub fn arg<T: Into<OsString>>(mut self, arg: T) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        for arg in args {
            self = self.arg(arg);
        }
        self
    }

    /// Consumes the builder and spawns `bitcoind`.
    ///
    /// # Panics
    ///
    /// If spawning the instance fails at any point.
    #[track_caller]
    pub fn spawn(self) -> BitcoinCoreInstance {
        let mut cmd = if let Some(ref prg) = self.program {
            Command::new(prg)
        } else {
            Command::new("bitcoind")
        };
        cmd.stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::inherit());

        cmd.arg("-regtest");
        // added so we can run parallel instances
        cmd.arg("-listen=0");
        // so we can use sendtoaddress from fresh instance
        cmd.arg("-fallbackfee=0.00001");

        let datadir = TempDir::new().expect("Couldn't create temp directory");
        cmd.arg(format!("-datadir={}", datadir.path().to_str().unwrap()));

        let rpcport = self.rpcport.unwrap_or_else(unused_port);

        cmd.arg(format!("-rpcport={rpcport}"));

        cmd.arg(format!("-rpcuser={BITCOIN_RPC_USER}"));
        cmd.arg(format!("-rpcpassword={BITCOIN_RPC_PASSWORD}"));

        cmd.args(self.args);

        let mut child = cmd.spawn().expect("Couldn't start bitcoin");

        let stdout = child.stdout.take().expect("Unable to get stdout for bitcoin child process");

        let start = Instant::now();
        let mut reader = BufReader::new(stdout);

        loop {
            if start + Duration::from_millis(self.timeout.unwrap_or(BITCOIN_STARTUP_TIMEOUT_MILLIS))
                <= Instant::now()
            {
                panic!("Timed out waiting for bitcoin to start. Is bitcoind installed?")
            }

            let mut line = String::new();
            reader.read_line(&mut line).expect("Failed to read line from bitcoin process");
            if line.contains("Done loading") {
                break;
            }
        }

        BitcoinCoreInstance { pid: child, _datadir: datadir, rpcport }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BitcoinClient, BumpFeeOptions};
    use bitcoin::Amount;

    #[test]
    fn can_launch_bitcoin() {
        let _ = BitcoinCore::new().spawn();
    }

    #[test]
    fn can_launch_bitcoin_with_more_accounts() {
        let bitcoin = BitcoinCore::new().spawn();
        bitcoin.fund_wallet("Alice").expect("Should fund Alice");
        bitcoin.fund_wallet("Bob").expect("Should fund Alice");
        assert_eq!(
            bitcoin.client(Some("Bob")).unwrap().get_balance(None, None).unwrap().to_sat(),
            5000000000
        );
    }

    #[tokio::test]
    async fn test_psbt_bump_fee() -> Result<()> {
        // Step 1: Create and initialize BitcoinCore instance for test
        let bitcoin = BitcoinCore::new().spawn();

        // Fund Alice's wallet
        bitcoin.fund_wallet("Alice").expect("Should fund Alice");

        // Fund Bob's wallet
        bitcoin.fund_wallet("Bob").expect("Should fund Alice");

        // Check that Bob's balance is 5000000000 satoshis (i.e., 5 BTC)
        assert_eq!(
            bitcoin.client(Some("Bob")).unwrap().get_balance(None, None).unwrap().to_sat(),
            5000000000
        );

        // Initialize BitcoinClient for Alice (make sure Alice's wallet is used)
        let bitcoin_client = BitcoinClient::from(bitcoin.client(Some("Alice"))?);

        let to_addr = bitcoin_client.rpc.get_new_address(None, None).unwrap().assume_checked();

        // Set the amount to send
        let amount = Amount::from_sat(100_000); // 0.001 BTC (adjust as necessary)

        // Send the transaction (low fee expected)
        let txid = bitcoin_client
            .rpc
            .send_to_address(&to_addr, amount, None, None, None, Some(true), None, None)
            .unwrap();

        // Step 3: Psbt Bump the fee for the low-fee transaction by calling bump_fee
        let psbt_bump_fee = bitcoin_client
            .psbt_bump_fee(
                &txid,
                Some(&BumpFeeOptions {
                    conf_target: None,
                    fee_rate: None,
                    replaceable: Some(true), // Allow the transaction to be replaceable
                    estimate_mode: None,
                }),
            )
            .unwrap();

        // the previous tx fee should be less than the newly created tx fee
        assert!(psbt_bump_fee.origfee < psbt_bump_fee.fee);

        // Sign and finalize the PSBT
        let tx = bitcoin_client.sign_and_finalize_psbt(
            &psbt_bump_fee.psbt.unwrap(),
            None,
            None,
            None,
        )?;

        // broadcast the bumped fee transaction
        bitcoin_client.validate_and_send_raw_transaction(&tx).unwrap();

        // Step 4: Generate 100 blocks to confirm the bump fee transaction
        bitcoin_client.rpc.generate_to_address(100, &to_addr).unwrap();

        // Check the original transaction
        let tx_info = bitcoin_client.rpc.get_transaction(&txid, None).unwrap();

        // Assert that the original transaction has negative confirmations
        assert!(tx_info.info.confirmations.is_negative());

        // Get the psbt bumped fee transaction's
        let tx_info = bitcoin_client.rpc.get_transaction(&tx.compute_txid(), None).unwrap();

        // Assert that the psbt bumped fee transaction has confirmations
        assert!(tx_info.info.confirmations.is_positive());

        Ok(())
    }

    #[tokio::test]
    async fn test_bump_fee() -> Result<()> {
        // Step 1: Create and initialize BitcoinCore instance for test
        let bitcoin = BitcoinCore::new().spawn();

        // Fund Alice's wallet
        bitcoin.fund_wallet("Alice").expect("Should fund Alice");

        // Fund Bob's wallet
        bitcoin.fund_wallet("Bob").expect("Should fund Alice");

        // Check that Bob's balance is 5000000000 satoshis (i.e., 5 BTC)
        assert_eq!(
            bitcoin.client(Some("Bob")).unwrap().get_balance(None, None).unwrap().to_sat(),
            5000000000
        );

        // Initialize BitcoinClient for Alice (make sure Alice's wallet is used)
        let bitcoin_client = BitcoinClient::from(bitcoin.client(Some("Alice"))?);

        // Step 2: Send to yourself with very low fee (simulate low fee transaction)
        let to_addr = bitcoin_client.rpc.get_new_address(None, None).unwrap().assume_checked();

        // Set the amount to send (you can adjust the amount as needed)
        let amount = Amount::from_sat(100_000); // 0.001 BTC (adjust as necessary)

        // Send the transaction to yourself (low fee expected)
        let txid = bitcoin_client
            .rpc
            .send_to_address(&to_addr, amount, None, None, None, Some(true), None, None)
            .unwrap();

        // Step 3: Bump the fee for the low-fee transaction by calling bump_fee
        let bump_fee = bitcoin_client
            .bump_fee(
                &txid,
                Some(&BumpFeeOptions {
                    conf_target: None,
                    fee_rate: None,
                    replaceable: Some(true), // Allow the transaction to be replaceable
                    estimate_mode: None,
                }),
            )
            .unwrap();

        // Assert there are no errors when bumping the fee
        assert!(bump_fee.errors.is_empty());

        // Step 4: Generate 100 blocks to confirm the bump fee transaction
        bitcoin_client.rpc.generate_to_address(100, &to_addr).unwrap();

        // Check the original transaction
        let tx_info = bitcoin_client.rpc.get_transaction(&txid, None).unwrap();

        // Assert that the original transaction has negative confirmations
        assert!(tx_info.info.confirmations.is_negative());

        // Get the bumped fee transaction's
        let tx_info = bitcoin_client.rpc.get_transaction(&bump_fee.txid.unwrap(), None).unwrap();

        // Assert that the bumped fee transaction has confirmations
        assert!(tx_info.info.confirmations.is_positive());

        Ok(())
    }
}
