use crate::core::cmd;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_docker() -> Result<InstallStatus> {
    if cmd::command_exists("docker") && cmd::command_exists("docker-compose") {
        let ver = version::get_docker_version();
        println!("{} Docker is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Docker CE & Docker Compose...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://get.docker.com | sudo sh"])?;
    
    println!("⏳ Adding current user to docker group...");
    let user = std::env::var("USER").unwrap_or_else(|_| "ubuntu".to_string());
    let _ = cmd::run_sudo_cmd("usermod", &["-aG", "docker", &user]);
    
    let ver = version::get_docker_version();
    println!("{} Docker installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
