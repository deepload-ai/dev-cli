use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use std::thread;

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

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn()
        .with_context(|| format!("Failed to spawn {}", cmd))?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let (tx_out, rx_out) = std::sync::mpsc::channel();
    let tx_err = tx_out.clone();

    // Spawn thread to read stdout
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = tx_out.send(l);
            }
        }
    });

    // Spawn thread to read stderr
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = tx_err.send(l);
            }
        }
    });

    let mut output_lines = Vec::with_capacity(50);
    let start_time = Instant::now();
    let timeout = Duration::from_secs(60); // 1 minute timeout before showing live logs
    let mut showing_live_logs = false;

    // Poll the channel for output lines
    loop {
        // Check if process has exited
        if let Ok(Some(status)) = child.try_wait() {
            // Drain remaining logs
            while let Ok(line) = rx_out.try_recv() {
                if showing_live_logs {
                    println!("    > {}", line);
                } else {
                    output_lines.push(line);
                    if output_lines.len() > 50 {
                        output_lines.remove(0);
                    }
                }
            }
            
            if !status.success() {
                let error_context = if output_lines.is_empty() {
                    "No output captured".to_string()
                } else {
                    format!("Last output:\n---\n{}\n---", output_lines.join("\n"))
                };
                anyhow::bail!("Command failed with status: {}\n{}", status, error_context);
            }
            break;
        }

        // Try to read a line from the channel
        match rx_out.recv_timeout(Duration::from_millis(100)) {
            Ok(line) => {
                if showing_live_logs {
                    println!("    > {}", line);
                } else {
                    output_lines.push(line);
                    if output_lines.len() > 50 {
                        output_lines.remove(0);
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // If we've been running for more than 1 minute and haven't started showing logs yet
                if !showing_live_logs && start_time.elapsed() > timeout {
                    showing_live_logs = true;
                    println!("\n  ⚠️ This step is taking longer than expected. Showing live logs:");
                    // Print the buffered lines first
                    for l in &output_lines {
                        println!("    > {}", l);
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                // Both threads have finished sending
            }
        }
    }

    Ok(())
}
