use std::process::Stdio;

use anyhow::{Context, Result};
use async_trait::async_trait;
use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines},
    process::{ChildStdout, Command},
};

const UNIT_NAME: &str = "app-dev.lizardbyte.app.Sunshine";

pub struct Provider {
    child_process: tokio::process::Child,
    lines: Lines<BufReader<ChildStdout>>,
}

impl Provider {
    pub fn new() -> Result<Self> {
        let mut child_process = Command::new("journalctl")
            .args([
                "--user", "-u", UNIT_NAME, "-f", "-o", "cat", "--since", "now",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to run journalctl process")?;

        let stdout = child_process
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("failed to capture journalctl stdout"))?;
        let reader = BufReader::new(stdout);

        let provider = Self {
            child_process,
            lines: reader.lines(),
        };

        Ok(provider)
    }
}

#[async_trait]
impl super::Log for Provider {
    async fn next_line(&mut self) -> Result<Option<String>> {
        let result = self.lines.next_line().await;

        result.context("failed to read journalctl line")
    }

    async fn terminate(&mut self) {
        let _ = self.child_process.kill().await;
    }
}
