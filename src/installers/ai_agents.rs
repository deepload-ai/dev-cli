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
    if !cmd::command_exists("git") {
        anyhow::bail!("git is required to install skills. Please install Git first.");
    }

    println!("⏳ Installing Claude Code via npm...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "@anthropic-ai/claude-code"]);

    println!("⏳ Installing Claude Code skills (ecc-universal, oh-my-claudecode)...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "ecc-universal"]);
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "oh-my-claude-sisyphus@latest"]);

    let home = get_home()?;
    let claude_skills_dir = Path::new(&home).join(".claude").join("skills");

    println!("⏳ Cloning gstack skills...");
    let gstack_path = claude_skills_dir.join("gstack");
    let _ = clone_skill_repo("https://github.com/garrytan/gstack.git", &gstack_path);

    println!("⏳ Cloning ui-ux-pro-max-skill...");
    let ui_ux_path = claude_skills_dir.join("ui-ux-pro-max-skill");
    let _ = clone_skill_repo("https://github.com/nextlevelbuilder/ui-ux-pro-max-skill.git", &ui_ux_path);

    let ver = version::get_generic_version("claude");
    println!("{} Claude Code and skills installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_codex() -> Result<InstallStatus> {
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install Codex. Please install Node.js first.");
    }
    if !cmd::command_exists("git") {
        anyhow::bail!("git is required to install skills. Please install Git first.");
    }

    println!("⏳ Installing Codex via npm...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "@openai/codex"]);

    println!("⏳ Installing Codex skills...");
    // Since Codex is an agent, we install the same recommended universal skills
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "ecc-universal"]);
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "oh-my-claude-sisyphus@latest"]);

    let home = get_home()?;
    let codex_skills_dir = Path::new(&home).join(".codex").join("skills");

    println!("⏳ Cloning gstack skills for Codex...");
    let gstack_path = codex_skills_dir.join("gstack");
    let _ = clone_skill_repo("https://github.com/garrytan/gstack.git", &gstack_path);

    println!("⏳ Cloning ui-ux-pro-max-skill for Codex...");
    let ui_ux_path = codex_skills_dir.join("ui-ux-pro-max-skill");
    let _ = clone_skill_repo("https://github.com/nextlevelbuilder/ui-ux-pro-max-skill.git", &ui_ux_path);

    let ver = version::get_generic_version("codex");
    println!("{} Codex and skills installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_opencode() -> Result<InstallStatus> {
    if !cmd::command_exists("curl") {
        anyhow::bail!("curl is required to install OpenCode. Please install curl first.");
    }
    if !cmd::command_exists("npm") {
        anyhow::bail!("npm is required to install OpenCode skills. Please install Node.js first.");
    }

    println!("⏳ Installing OpenCode via bash script...");
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

    println!("⏳ Installing OpenCode skills (oh-my-opencode)...");
    let _ = cmd::run_sudo_cmd("npm", &["install", "-g", "oh-my-opencode@latest"]);

    println!("⏳ Configuring OpenCode superpowers plugin...");
    let home = get_home()?;
    let opencode_config_dir = Path::new(&home).join(".config").join("opencode");
    if !opencode_config_dir.exists() {
        let _ = fs::create_dir_all(&opencode_config_dir);
    }
    
    let opencode_json_path = opencode_config_dir.join("opencode.json");
    let mut config_json = serde_json::json!({});
    
    if opencode_json_path.exists() {
        if let Ok(content) = fs::read_to_string(&opencode_json_path) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&content) {
                config_json = parsed;
            }
        }
    }

    // Add superpowers plugin to opencode.json
    let plugin_str = "superpowers@git+https://github.com/obra/superpowers.git";
    if let Some(obj) = config_json.as_object_mut() {
        let plugins = obj.entry("plugin").or_insert_with(|| serde_json::json!([]));
        if let Some(arr) = plugins.as_array_mut() {
            let has_plugin = arr.iter().any(|v| v.as_str() == Some(plugin_str));
            if !has_plugin {
                arr.push(serde_json::json!(plugin_str));
            }
        } else {
            // "plugin" exists but is not an array, overwrite it
            *plugins = serde_json::json!([plugin_str]);
        }
    }

    if let Ok(new_content) = serde_json::to_string_pretty(&config_json) {
        let _ = fs::write(&opencode_json_path, new_content);
    }

    let ver = version::get_generic_version("opencode");
    println!("{} OpenCode and skills installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
