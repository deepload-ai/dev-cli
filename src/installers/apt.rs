use crate::core::cmd;
use anyhow::Result;

pub fn update() -> Result<()> {
    cmd::run_sudo_cmd_with_env(
        "apt-get",
        &["update", "-y"],
        &[("DEBIAN_FRONTEND", "noninteractive")],
    )
}

pub fn install(packages: &[&str]) -> Result<()> {
    let mut args = vec!["install", "-y"];
    args.extend_from_slice(packages);
    cmd::run_sudo_cmd_with_env(
        "apt-get",
        &args,
        &[("DEBIAN_FRONTEND", "noninteractive")],
    )
}

pub fn remove(packages: &[&str]) -> Result<()> {
    let mut args = vec!["remove", "-y", "--purge"];
    args.extend_from_slice(packages);
    cmd::run_sudo_cmd_with_env(
        "apt-get",
        &args,
        &[("DEBIAN_FRONTEND", "noninteractive")],
    )
}

pub fn autoremove() -> Result<()> {
    cmd::run_sudo_cmd_with_env(
        "apt-get",
        &["autoremove", "-y"],
        &[("DEBIAN_FRONTEND", "noninteractive")],
    )
}
