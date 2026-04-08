use std::process::Command;

pub fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
    // If in dry-run mode, return a mocked version
    if std::env::var("DEVENV_DRY_RUN").is_ok() {
        return Some("v1.0.0-dry-run".to_string());
    }

    let output = Command::new(cmd).args(args).output().ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return Some(stdout.lines().next().unwrap_or("").to_string());
        }
        // Fallback to stderr for some tools (like java)
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !stderr.is_empty() {
            return Some(stderr.lines().next().unwrap_or("").to_string());
        }
    }
    None
}

pub fn get_node_version() -> String {
    get_version("node", &["-v"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_python_version() -> String {
    get_version("python3", &["--version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_rust_version() -> String {
    get_version("rustc", &["--version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_go_version() -> String {
    get_version("go", &["version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_java_version() -> String {
    get_version("java", &["-version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_docker_version() -> String {
    get_version("docker", &["--version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_bun_version() -> String {
    get_version("bun", &["--version"]).unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_generic_version(cmd: &str) -> String {
    get_version(cmd, &["--version"]).unwrap_or_else(|| "Unknown".to_string())
}
