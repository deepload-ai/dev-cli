use crate::core::cmd;
use super::apt;
use anyhow::{Context, Result};
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_nodejs() -> Result<InstallStatus> {
    if cmd::command_exists("node") && cmd::command_exists("npm") {
        println!("⏳ Checking Node.js installation...");
        // Ensure npm global directory is owned by user to prevent EACCES errors
        setup_npm_global_prefix()?;
        
        // Make sure pnpm is installed
        if !cmd::command_exists("pnpm") {
            cmd::run_cmd("npm", &["install", "-g", "pnpm"])?;
        }
        let ver = version::get_node_version();
        println!("{} Node.js is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    
    println!("⏳ Installing Node.js via NodeSource (LTS)...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -"])?;
    apt::install(&["nodejs"])?;
    
    setup_npm_global_prefix()?;
    
    cmd::run_cmd("npm", &["install", "-g", "pnpm"])?;
    let ver = version::get_node_version();
    println!("{} Node.js installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

fn setup_npm_global_prefix() -> Result<()> {
    // To solve EACCES issues when AI agents (like Claude Code) try to `npm install -g`
    // We change the default npm global directory to a user-owned directory (~/.npm-global)
    // and link its bin folder to /usr/local/bin so AI agents can still find the executables
    // without needing to source ~/.bashrc
    
    println!("🟢 Configuring npm global prefix to avoid sudo requirements...");
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let npm_global = format!("{}/.npm-global", home);
    
    cmd::run_cmd("mkdir", &["-p", &npm_global])?;
    cmd::run_cmd("npm", &["config", "set", "prefix", &npm_global])?;
    
    // We create a system-wide profile script to update PATH for login shells
    let script = format!("echo 'export PATH={}/bin:$PATH' | sudo tee /etc/profile.d/npm_global.sh > /dev/null", npm_global);
    cmd::run_cmd("bash", &["-c", &script])?;
    let _ = cmd::run_sudo_cmd("chmod", &["+x", "/etc/profile.d/npm_global.sh"]);
    
    // For non-interactive shells (AI Agents), we aggressively symlink the npm-global/bin directory
    // contents to /usr/local/bin whenever a new global package is installed.
    // However, since we can't hook into `npm install -g`, we'll at least link the currently known ones (like pnpm)
    // and provide a helper or rely on the fact that /etc/profile.d might be sourced by some agents.
    // Actually, a better approach for AI agents is to just make /usr/local/lib/node_modules and /usr/local/bin 
    // writable by the current user, or use the user-prefix. 
    // Let's stick to the user-prefix and link pnpm specifically since it's our default package manager.
    
    // Create the bin dir so symlinking doesn't fail if empty
    cmd::run_cmd("mkdir", &["-p", &format!("{}/bin", npm_global)])?;
    
    // Add ~/.npm-global/bin to PATH for the current process so subsequent `npm install -g` commands find it
    let current_path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}/bin:{}", npm_global, current_path));

    Ok(())
}

pub fn install_python() -> Result<InstallStatus> {
    if cmd::command_exists("python3") && cmd::command_exists("pip3") {
        println!("⏳ Checking Python installation...");
        setup_pip_user_base()?;
        let ver = version::get_python_version();
        println!("{} Python3 is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Python3...");
    let _ = cmd::run_sudo_cmd_with_env(
        "add-apt-repository", 
        &["ppa:deadsnakes/ppa", "-y"], 
        &[("DEBIAN_FRONTEND", "noninteractive")]
    );
    apt::update()?;
    
    // Install base python packages
    apt::install(&["python3", "python3-pip", "python3-dev", "python3-venv"])?;
    
    // Attempt to sniff exact Python version installed and install its specific venv package
    // to avoid issues where python3-venv meta-package fails to link to python3.x-venv correctly.
    if let Ok(output) = std::process::Command::new("python3").arg("-c").arg("import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')").output() {
        if output.status.success() {
            let py_ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !py_ver.is_empty() {
                let venv_pkg = format!("python{}-venv", py_ver);
                println!("⏳ Explicitly installing {}...", venv_pkg);
                let _ = apt::install(&[&venv_pkg]);
            }
        }
    }
    
    setup_pip_user_base()?;
    
    let ver = version::get_python_version();
    println!("{} Python3 installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

fn setup_pip_user_base() -> Result<()> {
    // To solve EACCES issues when AI agents run `pip install`
    // We explicitly instruct pip to use the user base (~/.local) instead of system directories.
    // We also make sure ~/.local/bin is globally sourced.
    
    println!("🐍 Configuring pip user base to avoid sudo requirements...");
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let local_bin = format!("{}/.local/bin", home);
    
    cmd::run_cmd("mkdir", &["-p", &local_bin])?;
    
    // Create pip.conf if it doesn't exist to force --user by default
    let pip_conf_dir = format!("{}/.config/pip", home);
    cmd::run_cmd("mkdir", &["-p", &pip_conf_dir])?;
    
    let pip_conf = format!("{}/pip.conf", pip_conf_dir);
    if !std::path::Path::new(&pip_conf).exists() {
        let conf_content = "[global]\nuser = true\n";
        std::fs::write(&pip_conf, conf_content)?;
    }
    
    // Add ~/.local/bin to PATH system-wide
    let script = format!("echo 'export PATH={}:$PATH' | sudo tee /etc/profile.d/pip_local.sh > /dev/null", local_bin);
    cmd::run_cmd("bash", &["-c", &script])?;
    let _ = cmd::run_sudo_cmd("chmod", &["+x", "/etc/profile.d/pip_local.sh"]);
    
    // Also update current process PATH
    let current_path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("{}:{}", local_bin, current_path));

    Ok(())
}

pub fn install_rust() -> Result<InstallStatus> {
    if cmd::command_exists("cargo") && cmd::command_exists("rustc") {
        let ver = version::get_rust_version();
        println!("{} Rust is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Rust (rustup, cargo)...");
    cmd::run_cmd("bash", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path"])?;
    
    // Link to /usr/local/bin to make it available for AI tools without ~/.bashrc loading
    println!("⏳ Linking Rust binaries to /usr/local/bin...");
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let bin_path = format!("{}/.cargo/bin", home);
    
    let bins = ["cargo", "rustc", "rustup", "rustfmt", "cargo-clippy"];
    for bin in bins {
        let src = format!("{}/{}", bin_path, bin);
        let dst = format!("/usr/local/bin/{}", bin);
        // Best effort linking
        let _ = cmd::run_sudo_cmd("ln", &["-sf", &src, &dst]);
    }
    
    let ver = version::get_rust_version();
    println!("{} Rust installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_bun() -> Result<InstallStatus> {
    if cmd::command_exists("bun") {
        let ver = version::get_bun_version();
        println!("{} Bun is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Bun...");
    cmd::run_cmd("bash", &["-c", "curl -fsSL https://bun.sh/install | bash"])?;
    
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let src = format!("{}/.bun/bin/bun", home);
    let dst = "/usr/local/bin/bun";
    let _ = cmd::run_sudo_cmd("ln", &["-sf", &src, dst]);
    
    let ver = version::get_bun_version();
    println!("{} Bun installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_java() -> Result<InstallStatus> {
    if cmd::command_exists("java") && cmd::command_exists("javac") {
        let ver = version::get_java_version();
        println!("{} Java is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Java (OpenJDK 17 LTS)...");
    apt::install(&["openjdk-17-jdk", "openjdk-17-jre"])?;
    
    // AI tools often look for JAVA_HOME. On Ubuntu, it's typically /usr/lib/jvm/java-17-openjdk-amd64 (or arm64)
    // We can set it globally in /etc/profile.d/java.sh
    let arch = std::process::Command::new("dpkg").arg("--print-architecture").output()?.stdout;
    let arch_str = String::from_utf8_lossy(&arch).trim().to_string();
    
    let java_home = format!("/usr/lib/jvm/java-17-openjdk-{}", arch_str);
    let script = format!("echo 'export JAVA_HOME={}' | sudo tee /etc/profile.d/jdk_home.sh > /dev/null", java_home);
    cmd::run_cmd("bash", &["-c", &script])?;
    let _ = cmd::run_sudo_cmd("chmod", &["+x", "/etc/profile.d/jdk_home.sh"]);
    
    let ver = version::get_java_version();
    println!("{} Java installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_android_sdk() -> Result<InstallStatus> {
    if cmd::command_exists("adb") && cmd::command_exists("sdkmanager") {
        let ver = version::get_generic_version("adb");
        println!("{} Android SDK is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Android SDK (Command line tools)...");
    // Ensure Java is installed first
    install_java().ok();
    
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let android_home = format!("{}/Android/Sdk", home);
    
    cmd::run_cmd("mkdir", &["-p", &format!("{}/cmdline-tools", android_home)])?;
    
    // Download latest command line tools (URL might need periodic update, this is a known stable one)
    let url = "https://dl.google.com/android/repository/commandlinetools-linux-10406996_latest.zip";
    let zip_path = "/tmp/cmdline-tools.zip";
    
    cmd::run_cmd("curl", &["-fsSL", url, "-o", zip_path])?;
    cmd::run_cmd("unzip", &["-q", "-o", zip_path, "-d", &format!("{}/cmdline-tools", android_home)])?;
    
    // The tools need to be placed in cmdline-tools/latest/bin
    let tools_dir = format!("{}/cmdline-tools/cmdline-tools", android_home);
    let latest_dir = format!("{}/cmdline-tools/latest", android_home);
    
    // Move cmdline-tools to latest if not already there
    let _ = cmd::run_cmd("mv", &[&tools_dir, &latest_dir]);
    
    // Set global environment variables
    let script = format!(
        "echo 'export ANDROID_HOME={}\nexport PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools' | sudo tee /etc/profile.d/android_home.sh > /dev/null", 
        android_home
    );
    cmd::run_cmd("bash", &["-c", &script])?;
    let _ = cmd::run_sudo_cmd("chmod", &["+x", "/etc/profile.d/android_home.sh"]);
    
    // Accept licenses
    println!("⏳ Accepting Android SDK licenses...");
    let sdkmanager = format!("{}/bin/sdkmanager", latest_dir);
    cmd::run_cmd("bash", &["-c", &format!("yes | {} --licenses", sdkmanager)])?;
    
    // Install basic platform tools
    cmd::run_cmd("bash", &["-c", &format!("{} \"platform-tools\" \"platforms;android-34\" \"build-tools;34.0.0\"", sdkmanager)])?;
    
    // Expose adb to AI
    let _ = cmd::run_sudo_cmd("ln", &["-sf", &format!("{}/platform-tools/adb", android_home), "/usr/local/bin/adb"]);

    let ver = version::get_generic_version("adb");
    println!("{} Android SDK installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_flutter() -> Result<InstallStatus> {
    if cmd::command_exists("flutter") && cmd::command_exists("dart") {
        let ver = version::get_generic_version("flutter");
        println!("{} Flutter SDK is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Flutter SDK...");
    
    let home = std::env::var("HOME").context("Failed to get HOME env")?;
    let flutter_dir = format!("{}/development/flutter", home);
    
    cmd::run_cmd("mkdir", &["-p", &format!("{}/development", home)])?;
    
    // Clone flutter stable branch
    let _ = cmd::run_cmd("git", &["clone", "https://github.com/flutter/flutter.git", "-b", "stable", &flutter_dir]);
    
    // Global link for AI agents
    let flutter_bin = format!("{}/bin/flutter", flutter_dir);
    let dart_bin = format!("{}/bin/dart", flutter_dir);
    
    let _ = cmd::run_sudo_cmd("ln", &["-sf", &flutter_bin, "/usr/local/bin/flutter"]);
    let _ = cmd::run_sudo_cmd("ln", &["-sf", &dart_bin, "/usr/local/bin/dart"]);
    
    // Pre-cache binaries
    println!("⏳ Precaching Flutter binaries (this may take a while)...");
    let _ = cmd::run_cmd("flutter", &["precache"]);
    
    let ver = version::get_generic_version("flutter");
    println!("{} Flutter SDK installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_go() -> Result<InstallStatus> {
    if cmd::command_exists("go") {
        let ver = version::get_go_version();
        println!("{} Go is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }

    println!("⏳ Installing Go (golang)...");
    let _ = cmd::run_sudo_cmd_with_env(
        "add-apt-repository", 
        &["ppa:longsleep/golang-backports", "-y"], 
        &[("DEBIAN_FRONTEND", "noninteractive")]
    );
    apt::update()?;
    apt::install(&["golang-go"])?;
    
    let ver = version::get_go_version();
    println!("{} Go installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
