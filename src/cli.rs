use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devenv-cli")]
#[command(about = "DevEnv CLI - Automated environment setup for Ubuntu, optimized for AI coding agents", long_about = None)]
#[command(author = "Trae Dev <dev@example.com>")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Install development tools and environments")]
    Install {
        #[arg(long, help = "Automatically install all default components")]
        auto: bool,
    },
    #[command(about = "Update all installed components to their latest versions")]
    Update,
    #[command(about = "Uninstall components and clean up the environment")]
    Uninstall,
    #[command(about = "List all supported components and their current installation status")]
    List,
}
