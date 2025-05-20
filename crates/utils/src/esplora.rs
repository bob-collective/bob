use crate::{unused_port, BitcoinCoreInstance, BITCOIN_RPC_PASSWORD, BITCOIN_RPC_USER};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command},
    time::{Duration, Instant},
};
use tempfile::TempDir;

const ESPLORA_STARTUP_TIMEOUT_MILLIS: u64 = 10_000;

pub struct EsploraInstance {
    pid: Child,
    _db_dir: TempDir,
    port: u16,
}

impl Drop for EsploraInstance {
    fn drop(&mut self) {
        self.pid.kill().expect("could not kill esplora");
    }
}

impl EsploraInstance {
    pub fn endpoint(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}

#[derive(Debug, Clone, Default)]
pub struct EsploraBuilder {
    program: Option<PathBuf>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl EsploraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(path: impl Into<PathBuf>) -> Self {
        Self::new().path(path)
    }

    pub fn path<T: Into<PathBuf>>(mut self, path: T) -> Self {
        self.program = Some(path.into());
        self
    }

    pub fn port<T: Into<u16>>(mut self, port: T) -> Self {
        self.port = Some(port.into());
        self
    }

    pub fn timeout<T: Into<u64>>(mut self, timeout: T) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    pub fn spawn(self, bitcoin: &BitcoinCoreInstance) -> EsploraInstance {
        self.spawn_with_port(bitcoin.rpcport())
    }

    pub fn spawn_with_port(self, bitcoin_port: u16) -> EsploraInstance {
        let mut cmd = self.program.as_ref().map_or_else(|| Command::new("electrs"), Command::new);
        cmd.stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());

        let http_port = self.port.unwrap_or_else(|| unused_port());
        let db_dir = TempDir::new().expect("Couldn't create temp directory");

        cmd.arg("-vvvv");
        cmd.arg("--network");
        cmd.arg("regtest");
        cmd.arg("--jsonrpc-import");
        cmd.arg("--cors");
        cmd.arg("*");
        cmd.arg("--cookie");
        cmd.arg(format!("{BITCOIN_RPC_USER}:{BITCOIN_RPC_PASSWORD}"));
        cmd.arg("--daemon-rpc-addr");
        cmd.arg(format!("127.0.0.1:{bitcoin_port}"));
        cmd.arg("--http-addr");
        cmd.arg(format!("[::0]:{}", http_port));
        cmd.arg("--index-unspendables");
        cmd.arg(format!("--db-dir={}", db_dir.path().to_str().unwrap()));
        cmd.arg("--electrum-rpc-addr=127.0.0.1:0");
        cmd.arg("--monitoring-addr=127.0.0.1:0");

        let mut child = cmd.spawn().expect("Couldn't start esplora");

        let stdout = child.stderr.take().expect("Unable to get stdout for esplora child process");

        let start = Instant::now();
        let mut reader = BufReader::new(stdout);

        loop {
            if start + Duration::from_millis(self.timeout.unwrap_or(ESPLORA_STARTUP_TIMEOUT_MILLIS))
                <= Instant::now()
            {
                panic!("Timed out waiting for esplora to start. Is electrs installed?")
            }

            let mut line = String::new();
            // TODO: this will block if no stdout (electrs uses stderr)
            reader.read_line(&mut line).expect("Failed to read line from esplora process");
            if line.contains("RPC server running") {
                break;
            }
        }

        EsploraInstance { pid: child, _db_dir: db_dir, port: http_port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BitcoinCore, EsploraClient};
    use eyre::Result;

    #[tokio::test]
    async fn can_launch_esplora() -> Result<()> {
        let bitcoin = BitcoinCore::new().spawn();
        bitcoin.fund_wallet("Alice").expect("Should fund Alice");

        let esplora = EsploraBuilder::new().spawn(&bitcoin);

        let esplora_client = EsploraClient::new_with_url(esplora.endpoint())?;

        let height = esplora_client.get_chain_height().await?;
        assert_eq!(101, height);
        Ok(())
    }
}
