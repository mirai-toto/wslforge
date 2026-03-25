use std::path::Path;

use clap::Parser;
use console::style;
use log::LevelFilter;
use wslforge::{app, cli::Args, config, wizard};

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    if args.print_example_config {
        println!("{}", config::EXAMPLE_CONFIG);
        return Ok(());
    }

    init_logger(args.log_file.as_deref())?;

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
            eprintln!("{}", style(format!("📄 Loaded config from '{}'", path.display())).dim());
            Ok(cfg)
        }
        None => {
            if default_path.exists() {
                eprintln!(
                    "{}",
                    style(format!(
                        "⚠️  No --config given, using '{}' found in current directory.",
                        default_path.display()
                    ))
                    .yellow()
                );
                let cfg = config::load_yaml(default_path)?;
                eprintln!(
                    "{}",
                    style(format!("📄 Loaded config from '{}'", default_path.display())).dim()
                );
                Ok(cfg)
            } else {
                wizard::run()
            }
        }
    }
}

fn init_logger(log_file: Option<&Path>) -> anyhow::Result<()> {
    let mut base = fern::Dispatch::new().level(LevelFilter::Off);

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
