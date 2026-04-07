use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Component {
    Base,
    BuildEssential,
    CMakeNinja,
    Sqlite3,
    NodeJs,
    Python,
    Rust,
    Go,
    Java,
    AndroidSdk,
    Flutter,
    Bun,
    Docker,
    Gh,
    Jq,
    Ripgrep,
    AITools,
    SentryCli,
}

impl Component {
    pub fn name(&self) -> &str {
        match self {
            Component::Base => "Base & Net (curl, git, zip, nc, psmisc...)",
            Component::BuildEssential => "Build Essential (gcc, make)",
            Component::CMakeNinja => "CMake & Ninja",
            Component::Sqlite3 => "SQLite3",
            Component::NodeJs => "Node.js (npm, pnpm)",
            Component::Python => "Python3 (pip, venv)",
            Component::Rust => "Rust (rustup, cargo)",
            Component::Go => "Go (golang)",
            Component::Java => "Java (OpenJDK 17)",
            Component::AndroidSdk => "Android SDK (cmdline-tools, adb)",
            Component::Flutter => "Flutter SDK",
            Component::Bun => "Bun",
            Component::Docker => "Docker & Docker Compose",
            Component::Gh => "GitHub CLI (gh)",
            Component::Jq => "jq",
            Component::Ripgrep => "ripgrep (rg)",
            Component::AITools => "AI Analysis Tools (bat, fd, tree, btop)",
            Component::SentryCli => "sentry-cli",
        }
    }

    pub fn all() -> Vec<Component> {
        vec![
            Component::Base,
            Component::BuildEssential,
            Component::CMakeNinja,
            Component::Sqlite3,
            Component::NodeJs,
            Component::Python,
            Component::Rust,
            Component::Go,
            Component::Java,
            Component::AndroidSdk,
            Component::Flutter,
            Component::Bun,
            Component::Docker,
            Component::Gh,
            Component::Jq,
            Component::Ripgrep,
            Component::AITools,
            Component::SentryCli,
        ]
    }
}

pub fn select_components() -> Result<Vec<Component>> {
    let options = Component::all();
    let items: Vec<String> = options.iter().map(|c| c.name().to_string()).collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select components to install (Space to toggle, Enter to confirm)")
        .items(&items)
        .defaults(&vec![true; items.len()])
        .interact()?;

    let selected_components = selections
        .into_iter()
        .map(|i| options[i].clone())
        .collect();

    Ok(selected_components)
}

pub fn confirm_keep_data() -> Result<bool> {
    let keep = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to KEEP user data and caches (e.g. ~/.npm, ~/.cargo, ~/.bun)?")
        .default(true)
        .interact()?;
    Ok(keep)
}
