use crate::core::cmd;
use crate::installers::apt;
use crate::systemd;
use anyhow::{Result};
use std::fs;
use std::env;

fn is_dry_run() -> bool {
    env::var("DEVENV_DRY_RUN").unwrap_or_default() == "1"
}

pub fn uninstall_all(keep_data: bool) -> Result<()> {
    println!("🗑️ Starting full uninstallation process...");

    // 1. Remove systemd timer
    systemd::remove_daily_update().ok();

    // 2. Remove apt packages
    println!("🗑️ Removing APT packages...");
    apt::remove(&[
        "nodejs",
        "python3",
        "python3-pip",
        "python3-venv",
        "golang-go",
        "docker-ce",
        "docker-ce-cli",
        "containerd.io",
        "gh",
        "jq",
        "ripgrep",
        "cmake",
        "ninja-build",
        "sqlite3",
        "bat",
        "fd-find",
        "tree",
        "btop",
        "htop",
        "netcat-openbsd",
    ]).ok();
    apt::autoremove().ok();

    // 3. Remove Rust
    println!("🗑️ Removing Rust...");
    cmd::run_cmd("rustup", &["self", "uninstall", "-y"]).ok();
    
    // 4. Remove Bun
    println!("🗑️ Removing Bun...");
    let home = std::env::var("HOME").unwrap_or_default();
    if !home.is_empty() {
        if is_dry_run() {
            println!("  [DRY RUN] > rm -rf {}/.bun", home);
        } else {
            let _ = fs::remove_dir_all(format!("{}/.bun", home));
        }
    }

    // 5. Remove links
    println!("🗑️ Removing global links...");
    let links = [
        "/usr/local/bin/cargo",
        "/usr/local/bin/rustc",
        "/usr/local/bin/rustup",
        "/usr/local/bin/rustfmt",
        "/usr/local/bin/cargo-clippy",
        "/usr/local/bin/bun",
        "/usr/local/bin/sentry-cli",
        "/usr/local/bin/bat",
        "/usr/local/bin/fd",
    ];
    for link in links {
        let _ = cmd::run_sudo_cmd("rm", &["-f", link]);
    }

    if !keep_data {
        println!("🔥 Performing deep cleanup of user data and caches...");
        if !home.is_empty() {
            if is_dry_run() {
                println!("  [DRY RUN] > rm -rf {}/.npm", home);
                println!("  [DRY RUN] > rm -rf {}/.cargo", home);
                println!("  [DRY RUN] > rm -rf {}/.rustup", home);
            } else {
                let _ = fs::remove_dir_all(format!("{}/.npm", home));
                let _ = fs::remove_dir_all(format!("{}/.cargo", home));
                let _ = fs::remove_dir_all(format!("{}/.rustup", home));
            }
        }
        // docker system prune -af --volumes
        let _ = cmd::run_sudo_cmd("docker", &["system", "prune", "-af", "--volumes"]);
    } else {
        println!("💾 User data and caches were preserved.");
    }

    println!("✅ Uninstallation completed successfully.");
    Ok(())
}
