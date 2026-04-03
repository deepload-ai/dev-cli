use crate::core::cmd;
use super::apt;
use anyhow::Result;

pub fn install_gh() -> Result<()> {
    println!("🐙 Installing GitHub CLI (gh)...");
    let script = r#"
        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
        sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
    "#;
    cmd::run_cmd("bash", &["-c", script])?;
    apt::update()?;
    apt::install(&["gh"])?;
    Ok(())
}

pub fn install_jq() -> Result<()> {
    println!("🔍 Installing jq...");
    apt::install(&["jq"])?;
    Ok(())
}

pub fn install_ripgrep() -> Result<()> {
    println!("🔎 Installing ripgrep (rg)...");
    apt::install(&["ripgrep"])?;
    Ok(())
}

pub fn install_sentry_cli() -> Result<()> {
    println!("🐛 Installing sentry-cli...");
    cmd::run_cmd("bash", &["-c", "curl -sL https://sentry.io/get-cli/ | sudo bash"])?;
    Ok(())
}

pub fn install_cmake_ninja() -> Result<()> {
    println!("🏗️ Installing CMake & Ninja...");
    apt::install(&["cmake", "ninja-build"])?;
    Ok(())
}

pub fn install_sqlite3() -> Result<()> {
    println!("🗄️ Installing SQLite3...");
    apt::install(&["sqlite3", "libsqlite3-dev"])?;
    Ok(())
}

pub fn install_ai_tools() -> Result<()> {
    println!("🤖 Installing AI Analysis Tools (bat, fd, tree, btop)...");
    // btop is available in ubuntu 22.04+, fallback to htop if not found is handled by apt if we just provide both or use a ppa.
    // For simplicity we try to install both btop and htop.
    apt::install(&["bat", "fd-find", "tree", "btop", "htop"])?;
    
    // Create symlinks for bat and fd as they are installed as batcat and fdfind by default on Ubuntu
    let _ = cmd::run_sudo_cmd("ln", &["-sf", "/usr/bin/batcat", "/usr/local/bin/bat"]);
    let _ = cmd::run_sudo_cmd("ln", &["-sf", "/usr/bin/fdfind", "/usr/local/bin/fd"]);
    
    Ok(())
}
