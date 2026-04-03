use super::apt;
use anyhow::Result;

pub fn install_base() -> Result<()> {
    println!("📦 Installing base system and network tools...");
    apt::install(&[
        "curl", "git", "wget", "gnupg", "ca-certificates", "software-properties-common",
        "unzip", "zip", "tar", "psmisc", "netcat-openbsd",
    ])
}

pub fn install_build_essential() -> Result<()> {
    println!("🛠️ Installing build-essential (gcc, make, pkg-config)...");
    apt::install(&["build-essential", "pkg-config", "libssl-dev"])
}
