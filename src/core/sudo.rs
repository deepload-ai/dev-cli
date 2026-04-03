use anyhow::{Context, Result};
use std::process::Command;
use std::env;

pub fn ensure_sudo() -> Result<()> {
    if env::var("DEVENV_DRY_RUN").unwrap_or_default() == "1" {
        println!("🔐 [DRY RUN] Bypassing sudo privilege check...");
        return Ok(());
    }

    println!("🔐 Requesting sudo privileges for system-level installations...");
    let status = Command::new("sudo")
        .arg("-v")
        .status()
        .context("Failed to request sudo privileges")?;

    if !status.success() {
        anyhow::bail!("Sudo privileges are required to proceed.");
    }
    Ok(())
}
