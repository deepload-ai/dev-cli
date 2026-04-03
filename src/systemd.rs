use crate::core::cmd;
use anyhow::{Context, Result};
use std::fs;
use std::env;

fn is_dry_run() -> bool {
    env::var("DEVENV_DRY_RUN").unwrap_or_default() == "1"
}

pub fn setup_daily_update() -> Result<()> {
    println!("🕒 Setting up daily auto-update via systemd user timer...");

    if is_dry_run() {
        println!("  [DRY RUN] > Would create systemd service and timer files in ~/.config/systemd/user");
        println!("  [DRY RUN] > systemctl --user daemon-reload");
        println!("  [DRY RUN] > systemctl --user enable --now devenv-update.timer");
        println!("✅ [DRY RUN] Daily auto-update configured successfully.");
        return Ok(());
    }

    let home = std::env::var("HOME").context("HOME not found")?;
    let systemd_dir = format!("{}/.config/systemd/user", home);
    fs::create_dir_all(&systemd_dir)?;

    // We assume the executable is placed in /usr/local/bin/devenv-cli
    let service_content = r#"
[Unit]
Description=DevEnv CLI Daily Update
After=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/local/bin/devenv-cli update
"#;

    let timer_content = r#"
[Unit]
Description=Timer for DevEnv CLI Daily Update

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
"#;

    fs::write(format!("{}/devenv-update.service", systemd_dir), service_content.trim())?;
    fs::write(format!("{}/devenv-update.timer", systemd_dir), timer_content.trim())?;

    cmd::run_cmd("systemctl", &["--user", "daemon-reload"])?;
    cmd::run_cmd("systemctl", &["--user", "enable", "--now", "devenv-update.timer"])?;

    println!("✅ Daily auto-update configured successfully.");
    Ok(())
}

pub fn remove_daily_update() -> Result<()> {
    println!("🛑 Removing daily auto-update timer...");
    if is_dry_run() {
        println!("  [DRY RUN] > systemctl --user disable --now devenv-update.timer");
        println!("  [DRY RUN] > rm ~/.config/systemd/user/devenv-update.*");
        return Ok(());
    }

    let _ = cmd::run_cmd("systemctl", &["--user", "disable", "--now", "devenv-update.timer"]);
    let home = std::env::var("HOME").context("HOME not found")?;
    let systemd_dir = format!("{}/.config/systemd/user", home);
    let _ = fs::remove_file(format!("{}/devenv-update.service", systemd_dir));
    let _ = fs::remove_file(format!("{}/devenv-update.timer", systemd_dir));
    let _ = cmd::run_cmd("systemctl", &["--user", "daemon-reload"]);
    Ok(())
}
