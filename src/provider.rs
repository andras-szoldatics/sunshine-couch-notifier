use anyhow::Result;
use async_trait::async_trait;

pub mod journalctl;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    None,
    Journalctl,
}

#[async_trait]
pub trait Log: Send {
    async fn next_line(&mut self) -> Result<Option<String>>;

    async fn terminate(&mut self) {}
}

struct NullProvider;

#[async_trait]
impl Log for NullProvider {
    async fn next_line(&mut self) -> Result<Option<String>> {
        Ok(None)
    }
}

pub fn new_log(mode: Mode) -> Result<Box<dyn Log>> {
    match mode {
        Mode::None => Ok(Box::new(NullProvider)),
        Mode::Journalctl => Ok(Box::new(journalctl::Provider::new()?)),
    }
}
