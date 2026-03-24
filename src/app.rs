use std::collections::BTreeMap;

use crate::{
    config::Config,
    display, reporting,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        EngineKind, InstanceResult, RunOptions, WslManager,
    },
};

pub struct AppArgs {
    pub config: Config,
    pub dry_run: bool,
    pub debug: bool,
}

pub fn run(cfg: AppArgs) -> anyhow::Result<()> {
    ensure_windows()?;

    let manager: WslManager = WslManager::new(build_engine(EngineKind::Cli));
    let options: RunOptions = RunOptions {
        dry_run: cfg.dry_run,
        debug: cfg.debug,
    };
    let config: Config = cfg.config;

    for (instance_name, instance) in &config.instances {
        reporting::log_config_summary(instance_name, instance);
    }

    manager.prepare_environment(options.dry_run)?;

    let mut results: BTreeMap<String, InstanceResult> = BTreeMap::new();
    for (instance_name, instance) in &config.instances {
        let pb = display::spinner(format!("Provisioning '{instance_name}'..."));
        let result = manager.create_instance(instance, options)?;
        pb.finish_and_clear();
        result.log();
        results.insert(instance_name.clone(), result);
    }

    display::print_summary(&results);

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
