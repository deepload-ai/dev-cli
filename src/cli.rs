use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devenv-cli")]
#[command(author = "Trae Dev")]
#[command(version = "0.1.0")]
#[command(about = "Ubuntu Development Environment Automated Setup", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install development tools and environments
    Install {
        /// Automatically install all default tools without prompting
        #[arg(short, long)]
        auto: bool,
    },
    /// Update all installed tools to their latest versions
    Update,
    /// Uninstall tools and optionally clean up data
    Uninstall,
}
