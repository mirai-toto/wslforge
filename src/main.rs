use std::path::Path;

use clap::Parser;
use log::LevelFilter;
use wslforge::{app, cli::Args, config, wizard};

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    if args.print_example_config {
        println!("{}", config::EXAMPLE_CONFIG);
        return Ok(());
    }

    init_logger(args.verbose);

    let loaded = resolve_config(args.config.as_deref())?;
    app::run(app::AppArgs {
        config: loaded,
        dry_run: args.dry_run,
        debug: args.debug,
    })?;

    Ok(())
}

fn resolve_config(explicit: Option<&Path>) -> anyhow::Result<config::Config> {
    let default_path = Path::new("config.yaml");

    match explicit {
        Some(path) => {
            let cfg = config::load_yaml(path)?;
            log::debug!("📋 Loaded config from {}", path.display());
            Ok(cfg)
        }
        None => {
            if default_path.exists() {
                log::warn!(
                    "No --config flag given; using '{}' found in current directory.",
                    default_path.display()
                );
                let cfg = config::load_yaml(default_path)?;
                log::debug!("📋 Loaded config from {}", default_path.display());
                Ok(cfg)
            } else {
                wizard::run()
            }
        }
    }
}

fn init_logger(verbosity: u8) {
    let level: LevelFilter = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp(None)
        .init();
}
