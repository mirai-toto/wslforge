use clap::{Parser, ValueHint};
use clap_complete::Shell;
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

    /// Write logs to a file at the given path (debug level with timestamps)
    #[arg(long, value_hint = ValueHint::FilePath)]
    pub log_file: Option<PathBuf>,

    /// Print a minimal example config to stdout and exit
    #[arg(long)]
    pub print_example_config: bool,

    /// Generate shell completion script and exit
    #[arg(long, value_name = "SHELL")]
    pub generate_completion: Option<Shell>,
}
