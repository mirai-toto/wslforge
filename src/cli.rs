use clap::{ArgAction, Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "wslforge", version, about = "Manage WSL instances from a YAML configuration")]
pub struct Args {
    /// Path to YAML configuration file
    #[arg(short, long, value_hint = ValueHint::FilePath, default_value = "config.yaml")]
    pub config: PathBuf,

    /// Show what would be done without creating the instance
    #[arg(long)]
    pub dry_run: bool,

    /// Enable extra debug output and artifacts
    #[arg(long)]
    pub debug: bool,

    /// Increase verbosity (-v, -vv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
