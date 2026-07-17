use crate::core::cmd;
use super::apt;
use super::lang;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_claude_code() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install Claude Code. Please install Node.js first.");
    }

    lang::setup_npm_global_prefix()?;
    println!("⏳ Installing/Updating Claude Code via npm...");
    cmd::run_cmd("npm", &["install", "-g", "@anthropic-ai/claude-code@latest"])?;

    let ver = version::get_generic_version("claude");
    println!("{} Claude Code installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_codex() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install Codex. Please install Node.js first.");
    }

    if !cmd::command_exists("bwrap") {
        println!("⏳ Installing bubblewrap (sandbox dependency for Codex)...");
        let _ = apt::update();
        let _ = apt::install(&["bubblewrap"]);
    }

    lang::setup_npm_global_prefix()?;
    println!("⏳ Installing/Updating Codex via npm...");
    cmd::run_cmd("npm", &["install", "-g", "@openai/codex@latest"])?;

    let ver = version::get_generic_version("codex");
    println!("{} Codex installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_opencode() -> Result<InstallStatus> {
    if !cmd::command_exists("curl") {
        anyhow::bail!("curl is required to install OpenCode. Please install curl first.");
    }

    if cmd::command_exists("opencode") {
        println!("✅ OpenCode is already installed. Attempting update via bash script...");
    } else {
        println!("⏳ Installing OpenCode via bash script...");
    }

    // curl -fsSL https://opencode.ai/install | bash
    let curl_output = std::process::Command::new("curl")
        .args(["-fsSL", "https://opencode.ai/install"])
        .output()?;

    if curl_output.status.success() {
        let script = String::from_utf8_lossy(&curl_output.stdout);
        let mut child = std::process::Command::new("bash")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(script.as_bytes());
        }
        let _ = child.wait()?;
    }

    let ver = version::get_generic_version("opencode");
    println!("{} OpenCode installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
