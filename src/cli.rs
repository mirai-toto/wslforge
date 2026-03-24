use clap::{ArgAction, Parser, ValueHint};
use std::path::PathBuf;

use crate::config;

#[derive(Parser, Debug)]
#[command(
    name = "wslforge",
    version,
    about = "Manage WSL instances from a YAML configuration",
    after_help = config::EXAMPLE_CONFIG
)]
pub struct Args {
    /// Path to YAML configuration file (defaults to config.yaml in current directory)
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    /// Show what would be done without creating the instance
    #[arg(long)]
    pub dry_run: bool,

    /// Enable extra debug output and write artifacts to the current directory (e.g. cloud-init.<hostname>.user-data)
    #[arg(long)]
    pub debug: bool,

    /// Increase verbosity (-v, -vv)
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,

    /// Write logs to a file at the given path (always at debug level with timestamps)
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub log_file: Option<PathBuf>,

    /// Print a minimal example config to stdout and exit
    #[arg(long)]
    pub print_example_config: bool,
}
