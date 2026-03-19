use std::path::Path;

use crate::{
    config, reporting,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        EngineKind, RunOptions, WslManager,
    },
};

pub struct AppArgs<'a> {
    pub config_path: &'a Path,
    pub dry_run: bool,
    pub debug: bool,
}

pub fn run(cfg: AppArgs<'_>) -> anyhow::Result<()> {
    ensure_windows()?;

    let manager = WslManager::new(build_engine(EngineKind::Cli));
    let options = RunOptions {
        dry_run: cfg.dry_run,
        debug: cfg.debug,
    };

    let config = config::load_yaml(cfg.config_path)?;
    log::debug!("📋 Loaded config from {}", cfg.config_path.display());

    for (instance_name, instance) in &config.instances {
        reporting::log_config_summary(instance_name, instance);
    }

    let results = manager.apply_config(&config, options)?;

    for instance_name in config.instances.keys() {
        let result = results
            .get(instance_name)
            .ok_or_else(|| anyhow::anyhow!("missing result for instance '{instance_name}'"))?;
        result.log();
    }

    Ok(())
}

fn ensure_windows() -> anyhow::Result<()> {
    if !cfg!(target_os = "windows") {
        anyhow::bail!("wslforge is Windows-only (target_os=windows required)");
    }
    Ok(())
}

fn build_engine(kind: EngineKind) -> Box<dyn WslEngine> {
    match kind {
        EngineKind::Cli => Box::new(CliEngine),
        EngineKind::Api => Box::new(ApiEngine),
    }
}
