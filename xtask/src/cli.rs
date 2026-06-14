use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xtask", about = "ChaOS build orchestrator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Path to the config file (defaults to the repo's chaos-config.toml)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    Build,
    Run,
}
