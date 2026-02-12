use std::path::Path;

use crate::{
    config, reporting,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        maintenance, EngineKind, ExecutionOptions, WslManager,
    },
};

pub struct AppConfig<'a> {
    pub config_path: &'a Path,
    pub dry_run: bool,
    pub debug: bool,
}

pub fn run(cfg: AppConfig<'_>) -> anyhow::Result<()> {
    ensure_windows()?;

    let manager = WslManager::new(build_engine(EngineKind::Cli));
    let options = ExecutionOptions {
        dry_run: cfg.dry_run,
        debug: cfg.debug,
    };
    let mut env_report = manager.validate_environment()?;
    let env_event = maintenance::environment::update_wsl_version(options.dry_run)?;
    env_report.events.push(env_event);
    reporting::log_environment_report(&env_report);

    let config = config::load_yaml(cfg.config_path)?;
    log::debug!("📋 Loaded config from {}", cfg.config_path.display());

    for (profile_name, profile) in &config.profiles {
        reporting::log_config_summary(profile_name, profile);
        let report = manager.create_instance(profile_name, profile, options)?;
        reporting::log_create_report(&report, &profile.hostname);
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
        EngineKind::Cli => Box::new(CliEngine::new()),
        EngineKind::Api => Box::new(ApiEngine::new()),
    }
}
