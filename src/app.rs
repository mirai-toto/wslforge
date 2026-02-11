use std::path::Path;

use crate::{config, wsl::cli, wsl::WslManager};

pub struct AppConfig<'a> {
    pub config_path: &'a Path,
    pub dry_run: bool,
    pub debug: bool,
}

pub fn run(cfg: AppConfig<'_>) -> anyhow::Result<()> {
    ensure_windows()?;

    let config = config::load_yaml(cfg.config_path)?;
    log::debug!("📋 Loaded config from {}", cfg.config_path.display());
    let manager = WslManager::new(cfg.dry_run, cfg.debug);

    for profile in config.profiles.values() {
        manager.validate_profile_config(profile)?;
    }
    let environment_report = manager.validate_environment()?;
    cli::log_environment_report(&environment_report);
    for (profile_name, profile) in &config.profiles {
        cli::log_config_summary(profile_name, profile);
        let report = manager.create_instance(profile_name, profile)?;
        cli::log_create_report(&report, &profile.hostname);
    }

    Ok(())
}

fn ensure_windows() -> anyhow::Result<()> {
    if !cfg!(target_os = "windows") {
        anyhow::bail!("wslforge is Windows-only (target_os=windows required)");
    }
    Ok(())
}
