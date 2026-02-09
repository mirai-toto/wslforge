use clap::Parser;
use log::LevelFilter;
use wslforge::{app, config, wsl::cli::Args};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.print_config {
        println!("{}", config::EXAMPLE_CONFIG);
        return Ok(());
    }

    init_logger(args.verbose);
    app::run(app::AppConfig {
        config_path: &args.config,
        dry_run: args.dry_run,
        debug: args.debug,
    })?;

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
