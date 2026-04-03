use crate::core::cmd;
use anyhow::Result;

pub fn install_docker() -> Result<()> {
    println!("🐳 Installing Docker and Docker Compose...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://get.docker.com | sudo sh"])?;
    
    let user = whoami::username();
    println!("🐳 Adding user '{}' to docker group...", user);
    cmd::run_sudo_cmd("usermod", &["-aG", "docker", &user])?;
    
    println!("⚠️ Note: You might need to logout and log back in for docker group changes to take effect.");
    Ok(())
}
