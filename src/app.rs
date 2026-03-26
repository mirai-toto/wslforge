use crate::{
    config::Config,
    display, reporting, wizard,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        EngineKind, RunOptions, WslManager,
    },
};

pub struct AppArgs {
    pub config: Config,
    pub dry_run: bool,
    pub debug: bool,
    pub force: bool,
}

pub fn run(cfg: AppArgs) -> anyhow::Result<()> {
    ensure_windows()?;

    let manager: WslManager = WslManager::new(build_engine(EngineKind::Cli));
    let options: RunOptions = RunOptions {
        dry_run: cfg.dry_run,
        debug: cfg.debug,
    };

    for (instance_name, instance) in &cfg.config.instances {
        reporting::log_config_summary(instance_name, instance);
    }

    if !cfg.force {
        wizard::confirm_provision()?;
    }

    let results = manager.provision_all(&cfg.config, options)?;
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
