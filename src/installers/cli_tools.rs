use crate::core::cmd;
use super::apt;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_gh() -> Result<InstallStatus> {
    if cmd::command_exists("gh") {
        let ver = version::get_generic_version("gh");
        println!("{} GitHub CLI is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing GitHub CLI (gh)...");
    let script = r#"
        curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
        sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
    "#;
    cmd::run_cmd("bash", &["-c", script])?;
    apt::update()?;
    apt::install(&["gh"])?;
    
    let ver = version::get_generic_version("gh");
    println!("{} GitHub CLI installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_jq() -> Result<InstallStatus> {
    if cmd::command_exists("jq") {
        let ver = version::get_generic_version("jq");
        println!("{} jq is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing jq...");
    apt::update()?;
    apt::install(&["jq"])?;
    
    let ver = version::get_generic_version("jq");
    println!("{} jq installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_ripgrep() -> Result<InstallStatus> {
    if cmd::command_exists("rg") {
        let ver = version::get_generic_version("rg");
        println!("{} ripgrep is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing ripgrep (rg)...");
    apt::update()?;
    apt::install(&["ripgrep"])?;
    
    let ver = version::get_generic_version("rg");
    println!("{} ripgrep installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_sentry_cli() -> Result<InstallStatus> {
    if cmd::command_exists("sentry-cli") {
        let ver = version::get_generic_version("sentry-cli");
        println!("{} sentry-cli is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing sentry-cli...");
    cmd::run_cmd("bash", &["-c", "curl -sL https://sentry.io/get-cli/ | sudo bash"])?;
    
    let ver = version::get_generic_version("sentry-cli");
    println!("{} sentry-cli installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_cmake_ninja() -> Result<InstallStatus> {
    if cmd::command_exists("cmake") && cmd::command_exists("ninja") {
        let ver = version::get_generic_version("cmake");
        println!("{} CMake & Ninja are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing CMake & Ninja...");
    
    // Always update apt before installing system packages to prevent 404 errors 
    apt::update()?;
    apt::install(&["cmake", "ninja-build"])?;
    
    let ver = version::get_generic_version("cmake");
    println!("{} CMake & Ninja installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_sqlite3() -> Result<InstallStatus> {
    if cmd::command_exists("sqlite3") {
        let ver = version::get_generic_version("sqlite3");
        println!("{} SQLite3 is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing SQLite3...");
    apt::update()?;
    apt::install(&["sqlite3", "libsqlite3-dev"])?;
    
    let ver = version::get_generic_version("sqlite3");
    println!("{} SQLite3 installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_ai_tools() -> Result<InstallStatus> {
    if cmd::command_exists("batcat") || cmd::command_exists("bat") {
        let ver = version::get_generic_version("bat");
        println!("{} AI Analysis Tools are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing AI Analysis Tools (bat, fd, tree, btop)...");
    // btop is available in ubuntu 22.04+, fallback to htop if not found is handled by apt if we just provide both or use a ppa.
    // For simplicity we try to install both btop and htop.
    apt::update()?;
    apt::install(&["bat", "fd-find", "tree", "btop", "htop"])?;
    
    // Create symlinks for bat and fd as they are installed as batcat and fdfind by default on Ubuntu
    let _ = cmd::run_sudo_cmd("ln", &["-sf", "/usr/bin/batcat", "/usr/local/bin/bat"]);
    let _ = cmd::run_sudo_cmd("ln", &["-sf", "/usr/bin/fdfind", "/usr/local/bin/fd"]);
    
    let ver = version::get_generic_version("bat");
    println!("{} AI Analysis Tools installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
