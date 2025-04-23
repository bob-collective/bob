use bitcoincore_rpc::{Auth, Client, RpcApi};
use eyre::Result;
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command},
    time::{Duration, Instant},
};
use tempfile::TempDir;

fn unused_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to create TCP listener to find unused port");

    let local_addr =
        listener.local_addr().expect("Failed to read TCP listener local_addr to find unused port");
    local_addr.port()
}

/// How long we will wait for bitcoin to indicate that it is ready.
const BITCOIN_STARTUP_TIMEOUT_MILLIS: u64 = 10_000;

const BITCOIN_RPC_USER: &str = "rpcuser";
const BITCOIN_RPC_PASSWORD: &str = "rpcpassword";

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
    args: Vec<String>,
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

        let rpcport = if let Some(rpcport) = self.rpcport { rpcport } else { unused_port() };
        cmd.arg(format!("-rpcport={}", rpcport));

        cmd.arg(format!("-rpcuser={}", BITCOIN_RPC_USER));
        cmd.arg(format!("-rpcpassword={}", BITCOIN_RPC_PASSWORD));

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
}
