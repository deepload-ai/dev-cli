# UX Enhancements: Install Status, Versions, and List Command Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the DevEnv CLI to provide rich, visual installation feedback with exact version numbers, and add a `list` command to display a summary table of all supported tools and their current status.

**Architecture:** 
1. Introduce a core `InstallStatus` enum and a `ToolInfo` struct to standardise the return type of all installer functions.
2. Implement a `version_checker` module that uses a trait/registry pattern to execute `--version` commands and parse strings for each tool.
3. Update `main.rs` to aggregate `InstallStatus` results and render a Markdown-style table.
4. Add a `list` subcommand to `cli.rs` that iterates through all `tui::Component`s, checks their versions, and renders the same table without installing.

**Tech Stack:** Rust, `clap` (for new CLI command), `console` (for colored icons like ✅, 🟢).

---

## Chunk 1: Define the Domain Models

**Files:**
- Create: `src/core/models.rs`
- Modify: `src/core/mod.rs`

- [ ] **Step 1: Create the domain models file**
Create `src/core/models.rs` with `InstallStatus` and `ToolInfo`:
```rust
use console::style;
use std::fmt;

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
```

- [ ] **Step 2: Export the models module**
Modify `src/core/mod.rs` to export the new module:
```rust
pub mod cmd;
pub mod sudo;
pub mod models;
```

## Chunk 2: Implement Version Checking Utility

**Files:**
- Create: `src/core/version.rs`
- Modify: `src/core/mod.rs`

- [ ] **Step 1: Create version extraction logic**
Create `src/core/version.rs` with helper functions to extract versions by running `--version` commands:
```rust
use anyhow::Result;
use std::process::Command;

pub fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
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

pub fn get_node_version() -> String { get_version("node", &["-v"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_python_version() -> String { get_version("python3", &["--version"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_rust_version() -> String { get_version("rustc", &["--version"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_go_version() -> String { get_version("go", &["version"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_java_version() -> String { get_version("java", &["-version"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_docker_version() -> String { get_version("docker", &["--version"]).unwrap_or_else(|| "Unknown".to_string()) }
pub fn get_generic_version(cmd: &str) -> String { get_version(cmd, &["--version"]).unwrap_or_else(|| "Unknown".to_string()) }
```

- [ ] **Step 2: Export version module**
Add `pub mod version;` to `src/core/mod.rs`.

## Chunk 3: Refactor Installers to Return InstallStatus

**Files:**
- Modify: `src/installers/lang.rs`
- Modify: `src/installers/cli_tools.rs`
- Modify: `src/installers/base.rs`
- Modify: `src/installers/docker.rs`

- [ ] **Step 1: Refactor `lang.rs`**
Update all functions in `lang.rs` to return `Result<crate::core::models::InstallStatus>`. Example for `install_nodejs`:
```rust
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_nodejs() -> Result<InstallStatus> {
    println!("⏳ Installing Node.js...");
    if cmd::command_exists("node") && cmd::command_exists("npm") {
        let ver = version::get_node_version();
        println!("{} Node.js is already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    // ... existing install logic ...
    let ver = version::get_node_version();
    println!("{} Node.js installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
```
*(Apply similar changes to Python, Rust, Go, Java, Android, Flutter, Bun)*

- [ ] **Step 2: Refactor `cli_tools.rs`, `base.rs`, `docker.rs`**
Change their return types to `Result<InstallStatus>`. Use `version::get_generic_version("jq")` etc. If a tool doesn't have a version command easily, return `InstallStatus::Installed("N/A".to_string())`.

## Chunk 4: Implement the Summary Table Renderer

**Files:**
- Create: `src/tui/summary.rs`
- Modify: `src/tui/mod.rs`

- [ ] **Step 1: Create the table renderer**
Create `src/tui/summary.rs`:
```rust
use crate::core::models::ToolInfo;
use console::style;

pub fn print_summary(tools: &[ToolInfo]) {
    println!("\n{}", style("==================================================================").cyan());
    println!("  {}", style("DevEnv CLI - Component Summary").bold().cyan());
    println!("{}", style("==================================================================").cyan());
    println!(" {:<20} | {:<18} | {}", "Component", "Status", "Version");
    println!("{}", style("---------------------+--------------------+-----------------------").dim());
    
    for tool in tools {
        let icon_status = format!("{} {}", tool.status.icon(), tool.status.text());
        println!(" {:<20} | {:<27} | {}", 
            style(&tool.name).bold(), 
            icon_status, 
            tool.status.version()
        );
    }
    println!("{}", style("==================================================================").cyan());
}
```

- [ ] **Step 2: Export summary module**
Add `pub mod summary;` to `src/tui/mod.rs`.

## Chunk 5: Add the `List` Command and Update `main.rs`

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: Add `List` subcommand**
In `src/cli.rs`:
```rust
#[derive(Subcommand)]
pub enum Commands {
    Install {
        #[arg(long, help = "Automatically install all default components")]
        auto: bool,
    },
    Update,
    Uninstall,
    #[command(about = "List all supported components and their current installation status")]
    List,
}
```

- [ ] **Step 2: Implement the `list` logic in `main.rs`**
In `src/main.rs`, add the `Commands::List` branch:
```rust
Commands::List => {
    let mut summary = Vec::new();
    for comp in tui::Component::all() {
        let name = format!("{:?}", comp);
        let (cmd_name, version_fn): (&str, fn() -> String) = match comp {
            tui::Component::NodeJs => ("node", crate::core::version::get_node_version),
            tui::Component::Python => ("python3", crate::core::version::get_python_version),
            tui::Component::Docker => ("docker", crate::core::version::get_docker_version),
            // Map others accordingly...
            _ => ("unknown", || "N/A".to_string()),
        };

        let status = if crate::core::cmd::command_exists(cmd_name) {
            crate::core::models::InstallStatus::AlreadyExists(version_fn())
        } else {
            crate::core::models::InstallStatus::NotInstalled
        };
        
        summary.push(crate::core::models::ToolInfo { name, status });
    }
    crate::tui::summary::print_summary(&summary);
}
```

- [ ] **Step 3: Update `Install` and `Update` logic to aggregate and print the summary**
In `main.rs` under `Commands::Install`:
```rust
let mut summary = Vec::new();
for comp in selections {
    let name = format!("{:?}", comp);
    let status = match comp {
        tui::Component::NodeJs => installers::lang::install_nodejs().unwrap_or_else(|e| crate::core::models::InstallStatus::Failed(e.to_string())),
        // ... match all other components ...
    };
    summary.push(crate::core::models::ToolInfo { name, status });
}
crate::tui::summary::print_summary(&summary);
```

## Chunk 6: Testing and Validation

- [ ] **Step 1: Test `list` command**
Run `cargo run -- list` to verify the table renders correctly with green icons for installed tools and dim icons for uninstalled ones.

- [ ] **Step 2: Test `install` output**
Run `cargo run -- install --auto` (with dry-run) to verify the progress logs `[⏳]` and the final summary table.