use std::path::Path;

use crate::{
    config, reporting,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        EngineKind, ExecutionOptions, WslManager,
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

    let config = config::load_yaml(cfg.config_path)?;
    log::debug!("📋 Loaded config from {}", cfg.config_path.display());

    for (profile_name, profile) in &config.profiles {
        reporting::log_config_summary(profile_name, profile);
    }

    let (environment_report, create_reports_by_profile) = manager.apply_config(&config, options)?;
    reporting::log_environment_report(&environment_report);

    for (profile_name, profile) in &config.profiles {
        let report = create_reports_by_profile
            .get(profile_name)
            .ok_or_else(|| anyhow::anyhow!("missing create report for profile '{profile_name}'"))?;
        reporting::log_create_report(report, &profile.hostname);
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
