use crate::core::cmd;
use super::apt;
use anyhow::{Context, Result};

pub fn install_nodejs() -> Result<()> {
    println!("🟢 Installing Node.js via NodeSource (LTS)...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -"])?;
    apt::install(&["nodejs"])?;
    
    println!("🟢 Installing pnpm...");
    cmd::run_sudo_cmd("npm", &["install", "-g", "pnpm"])?;
    Ok(())
}

pub fn install_python() -> Result<()> {
    println!("🐍 Installing Python3...");
    // Best effort to add deadsnakes PPA for latest Python versions
    let _ = cmd::run_sudo_cmd_with_env(
        "add-apt-repository", 
        &["ppa:deadsnakes/ppa", "-y"], 
        &[("DEBIAN_FRONTEND", "noninteractive")]
    );
    apt::update()?;
    apt::install(&["python3", "python3-pip", "python3-venv", "python3-dev"])?;
    Ok(())
}

pub fn install_rust() -> Result<()> {
    println!("🦀 Installing Rust (rustup, cargo)...");
    cmd::run_cmd("bash", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path"])?;
    
    // Link to /usr/local/bin to make it available for AI tools without ~/.bashrc loading
    println!("🦀 Linking Rust binaries to /usr/local/bin...");
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let bin_path = format!("{}/.cargo/bin", home);
    
    let bins = ["cargo", "rustc", "rustup", "rustfmt", "cargo-clippy"];
    for bin in bins {
        let src = format!("{}/{}", bin_path, bin);
        let dst = format!("/usr/local/bin/{}", bin);
        // Best effort linking
        let _ = cmd::run_sudo_cmd("ln", &["-sf", &src, &dst]);
    }
    
    Ok(())
}

pub fn install_bun() -> Result<()> {
    println!("🥟 Installing Bun...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://bun.sh/install | bash"])?;
    
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let src = format!("{}/.bun/bin/bun", home);
    let dst = "/usr/local/bin/bun";
    let _ = cmd::run_sudo_cmd("ln", &["-sf", &src, dst]);
    
    Ok(())
}

pub fn install_go() -> Result<()> {
    println!("🐹 Installing Go (golang)...");
    let _ = cmd::run_sudo_cmd_with_env(
        "add-apt-repository", 
        &["ppa:longsleep/golang-backports", "-y"], 
        &[("DEBIAN_FRONTEND", "noninteractive")]
    );
    apt::update()?;
    apt::install(&["golang-go"])?;
    Ok(())
}
