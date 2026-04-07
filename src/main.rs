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

            for comp in selections {
                match comp {
                    tui::Component::Base => installers::base::install_base()?,
                    tui::Component::BuildEssential => installers::base::install_build_essential()?,
                    tui::Component::CMakeNinja => installers::cli_tools::install_cmake_ninja()?,
                    tui::Component::Sqlite3 => installers::cli_tools::install_sqlite3()?,
                    tui::Component::NodeJs => installers::lang::install_nodejs()?,
                    tui::Component::Python => installers::lang::install_python()?,
                    tui::Component::Rust => installers::lang::install_rust()?,
                    tui::Component::Go => installers::lang::install_go()?,
                    tui::Component::Java => installers::lang::install_java()?,
                    tui::Component::AndroidSdk => installers::lang::install_android_sdk()?,
                    tui::Component::Flutter => installers::lang::install_flutter()?,
                    tui::Component::Bun => installers::lang::install_bun()?,
                    tui::Component::Docker => installers::docker::install_docker()?,
                    tui::Component::Gh => installers::cli_tools::install_gh()?,
                    tui::Component::Jq => installers::cli_tools::install_jq()?,
                    tui::Component::Ripgrep => installers::cli_tools::install_ripgrep()?,
                    tui::Component::AITools => installers::cli_tools::install_ai_tools()?,
                    tui::Component::SentryCli => installers::cli_tools::install_sentry_cli()?,
                }
            }

            println!("🎉 All selected components have been installed successfully!");
        }
        Commands::Update => {
            println!("🔄 Updating all components...");
            core::sudo::ensure_sudo()?;
            
            let _ = installers::apt::update();
            let _ = installers::apt::install(&["--only-upgrade", "nodejs", "python3", "docker-ce", "gh", "jq", "ripgrep"]);
            
            let _ = core::cmd::run_sudo_cmd("npm", &["update", "-g"]);
            let _ = core::cmd::run_cmd("rustup", &["update"]);
            let _ = core::cmd::run_cmd("bun", &["upgrade"]);
            let _ = core::cmd::run_sudo_cmd("sentry-cli", &["update"]);
            
            println!("✅ All components updated.");
        }
        Commands::Uninstall => {
            let keep_data = tui::confirm_keep_data()?;
            core::sudo::ensure_sudo()?;
            uninstaller::uninstall_all(keep_data)?;
        }
    }

    Ok(())
}
