use anyhow::{Result, bail};

use crate::provider::Mode;

pub fn provider_mode_from_args() -> Result<Mode> {
    let provider_mode_arg = std::env::args().nth(1);

    match provider_mode_arg.as_deref() {
        None | Some("journalctl") => Ok(Mode::Journalctl),
        Some("none") => Ok(Mode::None),
        Some(other) => bail!("invalid provider: '{other}', expected: none, journalctl"),
    }
}
