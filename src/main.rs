use clap::Parser;
use log::LevelFilter;
use wslforge::{cli::Args, config, wsl::WslManager};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger(args.verbose);

    let cfg = config::load_yaml(&args.config)?;
    log::debug!("📋 Loaded config: {:#?}", cfg);

    let manager = WslManager::new();

    if args.dry_run {
        manager.dry_run(&cfg)?;
    } else {
        manager.create_instance(&cfg)?;
    }

    Ok(())
}

fn init_logger(verbosity: u8) {
    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp(None)
        .init();
}
