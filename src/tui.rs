use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Component {
    // 1. Base system & C/C++ build tools
    Base,
    BuildEssential,
    CMakeNinja,
    Sqlite3,
    
    // 2. Core CLI utilities
    Jq,
    Ripgrep,
    AITools,
    SysDiag,
    DataTools,
    Gh,
    
    // 3. AI Environments & Dependencies
    AIMedia,
    WebAuto,
    
    // 4. Languages and runtimes
    NodeJs,
    Bun,
    Python,
    Rust,
    Go,
    Java,
    
    // 4. Heavy systems
    Docker,
    
    // 5. Mobile SDKs
    AndroidSdk,
    Flutter,
    
    // 6. Application level tools
    SentryCli,
    
    // 7. AI Coding Agents
    ClaudeCode,
    Codex,
    OpenCode,
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
            Component::SysDiag => "System Diagnostics (lsof, strace, dnsutils, etc)",
            Component::DataTools => "Data & Search Tools (yq, fzf)",
            Component::AIMedia => "AI Media & Docs (ffmpeg, imagemagick, poppler, tesseract)",
            Component::WebAuto => "Web Automation Deps (xvfb, libnss3, Playwright/Puppeteer deps)",
            Component::SentryCli => "sentry-cli",
            Component::ClaudeCode => "Claude Code (with gstack, ecc, ui-ux skills)",
            Component::Codex => "Codex (with skills)",
            Component::OpenCode => "OpenCode (with oh-my-opencode, superpowers)",
        }
    }

    pub fn all() -> Vec<Component> {
        vec![
            // 1. Base system & C/C++ build tools
            Component::Base,
            Component::BuildEssential,
            Component::CMakeNinja,
            Component::Sqlite3,
            
            // 2. Core CLI utilities
            Component::Jq,
            Component::Ripgrep,
            Component::AITools,
            Component::SysDiag,
            Component::DataTools,
            Component::Gh,
            
            // 3. AI Environments & Dependencies
            Component::AIMedia,
            Component::WebAuto,
            
            // 4. Languages and runtimes
            Component::NodeJs,
            Component::Bun,
            Component::Python,
            Component::Rust,
            Component::Go,
            Component::Java,
            
            // 4. Heavy systems
            Component::Docker,
            
            // 5. Mobile SDKs
            Component::AndroidSdk,
            Component::Flutter,
            
            // 6. Application level tools
            Component::SentryCli,
            
            // 7. AI Coding Agents
            Component::ClaudeCode,
            Component::Codex,
            Component::OpenCode,
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
