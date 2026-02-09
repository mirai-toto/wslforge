use std::path::Path;

use crate::{config, wsl::WslManager};

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

    manager.validate_environment()?;
    for (profile_name, profile) in &config.profiles {
        manager.create_instance(profile_name, profile)?;
    }

    Ok(())
}

fn ensure_windows() -> anyhow::Result<()> {
    if !cfg!(target_os = "windows") {
        anyhow::bail!("wslforge is Windows-only (target_os=windows required)");
    }
    Ok(())
}
