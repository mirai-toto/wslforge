use std::path::Path;

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use console::style;
use log::LevelFilter;
use std::io;
use wslforge::{app, cli::Args, config, wizard};

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    if args.print_example_config {
        println!("{}", config::EXAMPLE_CONFIG);
        return Ok(());
    }

    if let Some(shell) = args.generate_completion {
        generate(shell, &mut Args::command(), "wslforge", &mut io::stdout());
        return Ok(());
    }

    init_logger(args.log_file.as_deref())?;

    let loaded = resolve_config(args.config.as_deref())?;

    if args.validate {
        return app::validate(loaded);
    }

    app::run(app::AppArgs {
        config: loaded,
        dry_run: args.dry_run,
        debug: args.debug,
        force: args.force,
        cloud_init_timeout: args.cloud_init_timeout,
    })?;

    Ok(())
}

fn resolve_config(explicit: Option<&Path>) -> anyhow::Result<config::Config> {
    let default_path = Path::new("config.yaml");

    let path = match explicit {
        Some(path) => path,
        None if default_path.exists() => {
            eprintln!(
                "{}",
                style(format!(
                    "⚠️  No --config given, using '{}' found in current directory.",
                    default_path.display()
                ))
                .yellow()
            );
            default_path
        }
        None => return wizard::run(),
    };

    let cfg = config::load_yaml(path)?;
    eprintln!("{}", style(format!("📄 Loaded config from '{}'", path.display())).dim());
    Ok(cfg)
}

fn init_logger(log_file: Option<&Path>) -> anyhow::Result<()> {
    let mut base = fern::Dispatch::new();

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
