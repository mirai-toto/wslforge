use clap::Parser;
use log::LevelFilter;
use wslforge::{cli::Args, config, wsl::WslManager};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    init_logger(args.verbose);
    ensure_windows()?;

    let cfg = config::load_yaml(&args.config)?;
    log::debug!("📋 Loaded config from {}", args.config.display());
    let manager = WslManager::new();

    manager.validate_environment(args.dry_run)?;
    for (profile_name, profile) in &cfg.profiles {
        manager.create_instance(profile_name, profile, args.dry_run, args.debug)?;
    }

    Ok(())
}

fn ensure_windows() -> anyhow::Result<()> {
    if !cfg!(target_os = "windows") {
        anyhow::bail!("wslforge is Windows-only (target_os=windows required)");
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
