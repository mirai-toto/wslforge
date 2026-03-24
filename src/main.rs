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

    init_logger(args.verbose, args.log_file.as_deref())?;

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
            log::debug!("Loaded config from {}", path.display());
            Ok(cfg)
        }
        None => {
            if default_path.exists() {
                log::warn!(
                    "No --config flag given; using '{}' found in current directory.",
                    default_path.display()
                );
                let cfg = config::load_yaml(default_path)?;
                log::debug!("Loaded config from {}", default_path.display());
                Ok(cfg)
            } else {
                wizard::run()
            }
        }
    }
}

fn init_logger(verbosity: u8, log_file: Option<&Path>) -> anyhow::Result<()> {
    let stderr_level: LevelFilter = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    let stderr_dispatch = fern::Dispatch::new()
        .level(stderr_level)
        .format(|out, message, _record| out.finish(format_args!("{message}")))
        .chain(std::io::stderr());

    let mut base = fern::Dispatch::new().chain(stderr_dispatch);

    if let Some(path) = log_file {
        let file_dispatch = fern::Dispatch::new()
            .level(LevelFilter::Debug)
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{}] [{:<5}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    message
                ))
            })
            .chain(fern::log_file(path)?);
        base = base.chain(file_dispatch);
    }

    base.apply()?;
    Ok(())
}
