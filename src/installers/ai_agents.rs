use crate::core::cmd;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;
use std::env;
use std::path::Path;
use std::fs;

fn get_home() -> Result<String> {
    Ok(env::var("HOME").unwrap_or_else(|_| "/root".to_string()))
}

fn clone_skill_repo(repo_url: &str, dest_path: &Path) -> Result<()> {
    if dest_path.exists() {
        // If it already exists, maybe pull the latest?
        let dest_str = dest_path.to_str().unwrap();
        let _ = cmd::run_cmd("git", &["-C", dest_str, "pull"]);
    } else {
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }
        cmd::run_cmd("git", &["clone", repo_url, dest_path.to_str().unwrap()])?;
    }
    Ok(())
}

pub fn install_claude_code() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install Claude Code. Please install Node.js first.");
    }

    println!("⏳ Installing/Updating Claude Code via npm...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "@anthropic-ai/claude-code@latest"]);

    let ver = version::get_generic_version("claude");
    println!("{} Claude Code installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_codex() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install Codex. Please install Node.js first.");
    }

    println!("⏳ Installing/Updating Codex via npm...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "@openai/codex@latest"]);

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

pub fn install_ai_skills() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install skills. Please install Node.js first.");
    }
    if !cmd::command_exists("git") {
        anyhow::bail!("git is required to install skills. Please install Git first.");
    }

    let home = get_home()?;

    println!("⏳ Installing everything-claude-code (https://github.com/affaan-m/everything-claude-code)...");
    let ecc_path = Path::new(&home).join("everything-claude-code");
    if ecc_path.exists() {
        let _ = fs::remove_dir_all(&ecc_path);
    }
    let _ = cmd::run_cmd("git", &["clone", "https://github.com/affaan-m/everything-claude-code.git", ecc_path.to_str().unwrap()]);
    let _ = cmd::run_cmd("bash", &["-c", &format!("cd {} && npm install && ./install.sh --profile full", ecc_path.to_str().unwrap())]);
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "ecc-universal"]);

    println!("⏳ Installing claude-mem (https://install.cmem.ai)...");
    if cmd::command_exists("claude") {
        let _ = cmd::run_cmd("npx", &["-y", "claude-mem", "install"]);
    }
    if cmd::command_exists("opencode") {
        let _ = cmd::run_cmd("npx", &["-y", "claude-mem", "install", "--ide", "opencode"]);
    }

    println!("⏳ Installing openclaw (https://install.cmem.ai)...");
    let _ = cmd::run_cmd("bash", &["-c", "curl -fsSL https://install.cmem.ai/openclaw.sh | bash"]);

    println!("⏳ Installing rtk (https://github.com/rtk-ai/rtk)...");
    let _ = cmd::run_cmd("bash", &["-c", "curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh"]);

    println!("⏳ Installing pua (https://github.com/tanweai/pua)...");
    let _ = cmd::run_cmd("npx", &["-y", "skills", "add", "tanweai/pua", "--skill", "pua"]);

    println!("⏳ Installing gstack (https://github.com/garrytan/gstack)...");
    let gstack_path = Path::new(&home).join("gstack");
    if !gstack_path.exists() {
        let _ = cmd::run_cmd("git", &["clone", "--single-branch", "--depth", "1", "https://github.com/garrytan/gstack.git", gstack_path.to_str().unwrap()]);
    }
    let _ = cmd::run_cmd("bash", &["-c", &format!("cd {} && ./setup", gstack_path.to_str().unwrap())]);

    // Codex specific gstack setup
    if cmd::command_exists("codex") {
        let codex_skills_dir = Path::new(&home).join(".codex").join("skills");
        let gstack_path_codex = codex_skills_dir.join("gstack");
        let _ = clone_skill_repo("https://github.com/garrytan/gstack.git", &gstack_path_codex);
    }

    println!("⏳ Installing ui-ux-pro-max-skill (https://github.com/nextlevelbuilder/ui-ux-pro-max-skill)...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "uipro-cli"]);
    if cmd::command_exists("claude") {
        let _ = cmd::run_cmd("uipro", &["init", "--ai", "claude"]);
    }
    if cmd::command_exists("opencode") {
        let _ = cmd::run_cmd("uipro", &["init", "--ai", "opencode"]);
    }
    if cmd::command_exists("codex") {
        let codex_skills_dir = Path::new(&home).join(".codex").join("skills");
        let ui_ux_path_codex = codex_skills_dir.join("ui-ux-pro-max-skill");
        let _ = clone_skill_repo("https://github.com/nextlevelbuilder/ui-ux-pro-max-skill.git", &ui_ux_path_codex);
    }

    println!("⏳ Installing oh-my-claudecode (https://github.com/Yeachan-Heo/oh-my-claudecode)...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "oh-my-claude-sisyphus@latest"]);
    if cmd::command_exists("claude") {
        let _ = cmd::run_cmd("omc", &["setup"]);
    }

    println!("⏳ Installing graphify (https://github.com/safishamsi/graphify)...");
    let _ = cmd::run_cmd("bash", &["-c", "export PATH=$PATH:~/.local/bin && (pipx install graphifyy || pip install --user graphifyy)"]);
    if cmd::command_exists("claude") {
        let _ = cmd::run_cmd("bash", &["-c", "export PATH=$PATH:~/.local/bin && graphify install"]);
    }
    if cmd::command_exists("opencode") {
        let _ = cmd::run_cmd("bash", &["-c", "export PATH=$PATH:~/.local/bin && graphify install --platform opencode"]);
    }
    let _ = cmd::run_cmd("bash", &["-c", "export PATH=$PATH:~/.local/bin && graphify install --platform trae"]);

    println!("{} AI Agent Skills installed successfully", InstallStatus::Installed(String::new()).icon());
    Ok(InstallStatus::Installed("latest".to_string()))
}
