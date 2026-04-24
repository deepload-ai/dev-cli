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
                    tui::Component::SysDiag => installers::cli_tools::install_sys_diag(),
                    tui::Component::DataTools => installers::cli_tools::install_data_tools(),
                    tui::Component::AIMedia => installers::ai_env::install_ai_media(),
                    tui::Component::WebAuto => installers::ai_env::install_web_auto(),
                    tui::Component::SentryCli => installers::cli_tools::install_sentry_cli(),
                    tui::Component::ClaudeCode => installers::ai_agents::install_claude_code(),
                    tui::Component::Codex => installers::ai_agents::install_codex(),
                    tui::Component::OpenCode => installers::ai_agents::install_opencode(),
                };
                
                let status_val = match status {
                    Ok(s) => s,
                    Err(e) => {
                        let err_msg = e.to_string();
                        println!("{} {} failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), name, err_msg);
                        crate::core::models::InstallStatus::Failed(err_msg)
                    }
                };
                summary.push(crate::core::models::ToolInfo { name, status: status_val });
            }

            println!("🎉 All selected components have been processed!");
            crate::summary::print_summary(&summary);
        }
        Commands::Update => {
            println!("🔄 Updating all components...");
            core::sudo::ensure_sudo()?;
            
            let mut summary = Vec::new();
            
            println!("⏳ Updating APT Packages...");
            let apt_res = installers::apt::update()
                .and_then(|_| installers::apt::install(&["--only-upgrade", "nodejs", "python3", "docker-ce", "gh", "jq", "ripgrep"]));
            
            summary.push(crate::core::models::ToolInfo {
                name: "APT Packages".to_string(),
                status: if let Err(e) = &apt_res {
                    println!("{} APT Packages update failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), e);
                    crate::core::models::InstallStatus::Failed(e.to_string())
                } else {
                    println!("{} APT Packages updated successfully", crate::core::models::InstallStatus::Updated(String::new()).icon());
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                },
            });
            
            println!("⏳ Updating NPM Global Packages...");
            let npm_res = core::cmd::run_sudo_cmd("npm", &["update", "-g"]);
            summary.push(crate::core::models::ToolInfo {
                name: "NPM Global".to_string(),
                status: if let Err(e) = &npm_res {
                    println!("{} NPM Global update failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), e);
                    crate::core::models::InstallStatus::Failed(e.to_string())
                } else {
                    println!("{} NPM Global updated successfully", crate::core::models::InstallStatus::Updated(String::new()).icon());
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                },
            });

            println!("⏳ Updating Rustup...");
            let rust_res = core::cmd::run_cmd("rustup", &["update"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Rustup".to_string(),
                status: if let Err(e) = &rust_res {
                    println!("{} Rustup update failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), e);
                    crate::core::models::InstallStatus::Failed(e.to_string())
                } else {
                    println!("{} Rustup updated successfully", crate::core::models::InstallStatus::Updated(String::new()).icon());
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                },
            });

            println!("⏳ Updating Bun...");
            let bun_res = core::cmd::run_cmd("bun", &["upgrade"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Bun".to_string(),
                status: if let Err(e) = &bun_res {
                    println!("{} Bun update failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), e);
                    crate::core::models::InstallStatus::Failed(e.to_string())
                } else {
                    println!("{} Bun updated successfully", crate::core::models::InstallStatus::Updated(String::new()).icon());
                    crate::core::models::InstallStatus::Updated("latest".to_string())
                },
            });

            println!("⏳ Updating Sentry CLI...");
            let sentry_res = core::cmd::run_sudo_cmd("sentry-cli", &["update"]);
            summary.push(crate::core::models::ToolInfo {
                name: "Sentry CLI".to_string(),
                status: if let Err(e) = &sentry_res {
                    println!("{} Sentry CLI update failed: {}", crate::core::models::InstallStatus::Failed(String::new()).icon(), e);
                    crate::core::models::InstallStatus::Failed(e.to_string())
                } else {
                    println!("{} Sentry CLI updated successfully", crate::core::models::InstallStatus::Updated(String::new()).icon());
                    crate::core::models::InstallStatus::Updated("latest".to_string())
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
                    tui::Component::SysDiag => ("lsof", || crate::core::version::get_generic_version("lsof")),
                    tui::Component::DataTools => ("yq", || crate::core::version::get_generic_version("yq")),
                    tui::Component::AIMedia => ("ffmpeg", || crate::core::version::get_generic_version("ffmpeg")),
                    tui::Component::WebAuto => ("xvfb-run", || crate::core::version::get_generic_version("xvfb-run")),
                    tui::Component::SentryCli => ("sentry-cli", || crate::core::version::get_generic_version("sentry-cli")),
                    tui::Component::CMakeNinja => ("cmake", || crate::core::version::get_generic_version("cmake")),
                    tui::Component::Sqlite3 => ("sqlite3", || crate::core::version::get_generic_version("sqlite3")),
                    tui::Component::BuildEssential => ("gcc", || crate::core::version::get_generic_version("gcc")),
                    tui::Component::Base => ("git", || crate::core::version::get_generic_version("git")),
                    tui::Component::ClaudeCode => ("claude", || crate::core::version::get_generic_version("claude")),
                    tui::Component::Codex => ("codex", || crate::core::version::get_generic_version("codex")),
                    tui::Component::OpenCode => ("opencode", || crate::core::version::get_generic_version("opencode")),
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
