use console::style;

#[derive(Debug, Clone, PartialEq)]
pub enum InstallStatus {
    Installed(String),     // version
    AlreadyExists(String), // version
    Updated(String),       // version
    Failed(String),        // error message
    NotInstalled,          // For the `list` command
}

impl InstallStatus {
    pub fn icon(&self) -> String {
        match self {
            Self::Installed(_) => style("✅").green().to_string(),
            Self::AlreadyExists(_) => style("🟢").blue().to_string(),
            Self::Updated(_) => style("🔄").yellow().to_string(),
            Self::Failed(_) => style("❌").red().to_string(),
            Self::NotInstalled => style("⚪").dim().to_string(),
        }
    }

    pub fn text(&self) -> String {
        match self {
            Self::Installed(_) => "Installed".to_string(),
            Self::AlreadyExists(_) => "Already Exists".to_string(),
            Self::Updated(_) => "Updated".to_string(),
            Self::Failed(_) => "Failed".to_string(),
            Self::NotInstalled => "Not Installed".to_string(),
        }
    }

    pub fn version(&self) -> String {
        match self {
            Self::Installed(v) | Self::AlreadyExists(v) | Self::Updated(v) => v.clone(),
            Self::Failed(e) => format!("Error: {}", e),
            Self::NotInstalled => "N/A".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ToolInfo {
    pub name: String,
    pub status: InstallStatus,
}
