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
    run_cmd_internal(cmd, args, false, &[])
}

pub fn run_sudo_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    run_cmd_internal(cmd, args, true, &[])
}

pub fn run_cmd_with_env(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<()> {
    run_cmd_internal(cmd, args, false, envs)
}

pub fn run_sudo_cmd_with_env(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> Result<()> {
    run_cmd_internal(cmd, args, true, envs)
}

fn run_cmd_internal(cmd: &str, args: &[&str], sudo: bool, envs: &[(&str, &str)]) -> Result<()> {
    if is_dry_run() {
        let env_str = envs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" ");
        println!("  [DRY RUN] > {}{} {} {}", if sudo { "sudo " } else { "" }, env_str, cmd, args.join(" "));
        return Ok(());
    }

    let mut command = if sudo {
        let mut c = Command::new("sudo");
        c.arg("-E").arg(cmd);
        c
    } else {
        Command::new(cmd)
    };

    command.args(args);
    for (k, v) in envs {
        command.env(k, v);
    }

    // Use output() to capture stdout and stderr without deadlocking
    let output = command.output()
        .with_context(|| format!("Failed to execute {}", cmd))?;

    if !output.status.success() {
        // Combine stdout and stderr for the error context
        let mut all_output = String::from_utf8_lossy(&output.stdout).to_string();
        all_output.push_str("\n");
        all_output.push_str(&String::from_utf8_lossy(&output.stderr));
        
        let lines: Vec<&str> = all_output.lines().collect();
        
        // Take the last 50 lines
        let last_lines = if lines.len() > 50 {
            lines[lines.len() - 50..].join("\n")
        } else {
            lines.join("\n")
        };
        
        let error_context = if last_lines.trim().is_empty() {
            "No output captured".to_string()
        } else {
            format!("Last output:\n---\n{}\n---", last_lines)
        };
        anyhow::bail!("Command failed with status: {}\n{}", output.status, error_context);
    }

    Ok(())
}
