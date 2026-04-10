use crate::core::cmd;
use super::apt;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_base() -> Result<InstallStatus> {
    let required_cmds = ["curl", "git", "wget", "unzip", "zip", "tar"];
    let all_exist = required_cmds.iter().all(|&cmd| cmd::command_exists(cmd));
    
    if all_exist {
        let ver = version::get_generic_version("git");
        println!("{} Base utilities (curl, git, unzip, etc) are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing base utilities...");
    apt::install(&[
        "curl", "git", "wget", "gnupg", "ca-certificates", "software-properties-common",
        "unzip", "zip", "tar", "psmisc", "netcat-openbsd",
    ])?;
    
    let ver = version::get_generic_version("git");
    println!("{} Base utilities installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_build_essential() -> Result<InstallStatus> {
    let required_cmds = ["gcc", "make", "pkg-config"];
    let all_exist = required_cmds.iter().all(|&cmd| cmd::command_exists(cmd));
    
    if all_exist {
        let ver = version::get_generic_version("gcc");
        println!("{} Build essential tools are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    println!("⏳ Installing build-essential...");
    apt::install(&["build-essential", "pkg-config", "libssl-dev"])?;
    
    let ver = version::get_generic_version("gcc");
    println!("{} Build essential tools installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
