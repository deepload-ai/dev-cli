mod cli;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use devenv_cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install { auto } => {
            let selections = if *auto {
                println!("🚀 Auto-installing all default components...");
                tui::Component::all()
            } else {
                let s = tui::select_components()?;
                if s.is_empty() {
                    println!("No components selected. Exiting.");
                    return Ok(());
                }
                println!("🚀 Installing selected components...");
                s
            };

            // Request sudo upfront
            core::sudo::ensure_sudo()?;
            
            // Setup daily update
            let _ = systemd::setup_daily_update();

            let mut summary = Vec::new();

            for comp in selections {
                let name = format!("{:?}", comp);
                let status = match comp {
                    tui::Component::Base => installers::base::install_base(),
                    tui::Component::BuildEssential => installers::base::install_build_essential(),
                    tui::Component::CMakeNinja => installers::cli_tools::install_cmake_ninja(),
                    tui::Component::Sqlite3 => installers::cli_tools::install_sqlite3(),
                    tui::Component::NodeJs => installers::lang::install_nodejs(),
                    tui::Component::Python => installers::lang::install_python(),
                    tui::Component::Rust => installers::lang::install_rust(),
                    tui::Component::Go => installers::lang::install_go(),
                    tui::Component::Java => installers::lang::install_java(),
                    tui::Component::AndroidSdk => installers::lang::install_android_sdk(),
                    tui::Component::Flutter => installers::lang::install_flutter(),
                    tui::Component::Bun => installers::lang::install_bun(),
                    tui::Component::Docker => installers::docker::install_docker(),
                    tui::Component::Gh => installers::cli_tools::install_gh(),
                    tui::Component::Jq => installers::cli_tools::install_jq(),
                    tui::Component::Ripgrep => installers::cli_tools::install_ripgrep(),
                    tui::Component::AITools => installers::cli_tools::install_ai_tools(),
                    tui::Component::SentryCli => installers::cli_tools::install_sentry_cli(),
                };
                
                let status_val = status.unwrap_or_else(|e| crate::core::models::InstallStatus::Failed(e.to_string()));
                summary.push(crate::core::models::ToolInfo { name, status: status_val });
            }

            println!("🎉 All selected components have been processed!");
            crate::summary::print_summary(&summary);
        }
        Commands::Update => {
            println!("🔄 Updating all components...");
            core::sudo::ensure_sudo()?;
            
            let mut summary = Vec::new();
            
            let apt_res = installers::apt::update()
                .and_then(|_| installers::apt::install(&["--only-upgrade", "nodejs", "python3", "docker-ce", "gh", "jq", "ripgrep"]));
            
            summary.push(crate::core::models::ToolInfo {
                name: "APT Packages".to_string(),
                status: if apt_res.is_ok() {
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                } else {
                    crate::core::models::InstallStatus::Failed("APT update failed".to_string())
                },
            });
            
            let npm_res = core::cmd::run_sudo_cmd("npm", &["update", "-g"]);
            summary.push(crate::core::models::ToolInfo {
                name: "NPM Global".to_string(),
                status: if npm_res.is_ok() {
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                } else {
                    crate::core::models::InstallStatus::Failed("NPM update failed".to_string())
                },
            });

            let rust_res = core::cmd::run_cmd("rustup", &["update"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Rustup".to_string(),
                status: if rust_res.is_ok() {
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                } else {
                    crate::core::models::InstallStatus::Failed("Rustup update failed".to_string())
                },
            });

            let bun_res = core::cmd::run_cmd("bun", &["upgrade"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Bun".to_string(),
                status: if bun_res.is_ok() {
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                } else {
                    crate::core::models::InstallStatus::Failed("Bun upgrade failed".to_string())
                },
            });

            let sentry_res = core::cmd::run_sudo_cmd("sentry-cli", &["update"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Sentry CLI".to_string(),
                status: if sentry_res.is_ok() {
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                } else {
                    crate::core::models::InstallStatus::Failed("Sentry CLI update failed".to_string())
                },
            });
            
            println!("✅ Update process completed.");
            crate::summary::print_summary(&summary);
        }
        Commands::Uninstall => {
            let keep_data = tui::confirm_keep_data()?;
            core::sudo::ensure_sudo()?;
            uninstaller::uninstall_all(keep_data)?;
        }
        Commands::List => {
            let mut summary = Vec::new();
            for comp in tui::Component::all() {
                let name = format!("{:?}", comp);
                
                let (cmd_name, version_fn): (&str, fn() -> String) = match comp {
                    tui::Component::NodeJs => ("node", crate::core::version::get_node_version),
                    tui::Component::Python => ("python3", crate::core::version::get_python_version),
                    tui::Component::Rust => ("rustc", crate::core::version::get_rust_version),
                    tui::Component::Go => ("go", crate::core::version::get_go_version),
                    tui::Component::Java => ("java", crate::core::version::get_java_version),
                    tui::Component::AndroidSdk => ("adb", || crate::core::version::get_generic_version("adb")),
                    tui::Component::Flutter => ("flutter", || crate::core::version::get_generic_version("flutter")),
                    tui::Component::Bun => ("bun", crate::core::version::get_bun_version),
                    tui::Component::Docker => ("docker", crate::core::version::get_docker_version),
                    tui::Component::Gh => ("gh", || crate::core::version::get_generic_version("gh")),
                    tui::Component::Jq => ("jq", || crate::core::version::get_generic_version("jq")),
                    tui::Component::Ripgrep => ("rg", || crate::core::version::get_generic_version("rg")),
                    tui::Component::AITools => ("bat", || crate::core::version::get_generic_version("bat")),
                    tui::Component::SentryCli => ("sentry-cli", || crate::core::version::get_generic_version("sentry-cli")),
                    tui::Component::CMakeNinja => ("cmake", || crate::core::version::get_generic_version("cmake")),
                    tui::Component::Sqlite3 => ("sqlite3", || crate::core::version::get_generic_version("sqlite3")),
                    tui::Component::BuildEssential => ("gcc", || crate::core::version::get_generic_version("gcc")),
                    tui::Component::Base => ("git", || crate::core::version::get_generic_version("git")),
                };

                let status = if crate::core::cmd::command_exists(cmd_name) {
                    crate::core::models::InstallStatus::AlreadyExists(version_fn())
                } else {
                    crate::core::models::InstallStatus::NotInstalled
                };
                
                summary.push(crate::core::models::ToolInfo { name, status });
            }
            crate::summary::print_summary(&summary);
        }
    }

    Ok(())
}
