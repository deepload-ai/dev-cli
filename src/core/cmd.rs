use anyhow::{Context, Result};
use std::process::Command;
use std::env;

fn is_dry_run() -> bool {
    env::var("DEVENV_DRY_RUN").unwrap_or_default() == "1"
}

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    if is_dry_run() {
        println!("  [DRY RUN] > {} {}", cmd, args.join(" "));
        return Ok(());
    }
    
    println!("  > {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute {}", cmd))?;

    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }
    Ok(())
}

pub fn run_sudo_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    if is_dry_run() {
        println!("  [DRY RUN] > sudo {} {}", cmd, args.join(" "));
        return Ok(());
    }
    
    let mut sudo_args = vec![cmd];
    sudo_args.extend_from_slice(args);
    run_cmd("sudo", &sudo_args)
}

pub fn run_cmd_with_env(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<()> {
    if is_dry_run() {
        let env_str = envs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" ");
        println!("  [DRY RUN] > {} {} {}", env_str, cmd, args.join(" "));
        return Ok(());
    }

    println!("  > {} {}", cmd, args.join(" "));
    let mut command = Command::new(cmd);
    command.args(args);
    for (k, v) in envs {
        command.env(k, v);
    }
    
    let status = command
        .status()
        .with_context(|| format!("Failed to execute {}", cmd))?;

    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }
    Ok(())
}

pub fn run_sudo_cmd_with_env(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<()> {
    if is_dry_run() {
        let env_str = envs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" ");
        println!("  [DRY RUN] > sudo -E {} {} {}", env_str, cmd, args.join(" "));
        return Ok(());
    }

    let mut command = Command::new("sudo");
    command.arg("-E");
    command.arg(cmd);
    command.args(args);
    for (k, v) in envs {
        command.env(k, v);
    }

    println!("  > sudo -E {} {}", cmd, args.join(" "));
    let status = command
        .status()
        .with_context(|| format!("Failed to execute sudo {}", cmd))?;

    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }
    Ok(())
}
