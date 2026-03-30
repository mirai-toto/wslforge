use inquire::Select;

use crate::{
    config::Config,
    display, reporting, wizard,
    wsl::{
        engine::{api::ApiEngine, cli::CliEngine, WslEngine},
        validation::instance::{cloud_init_required, validate_instance},
        EngineKind, RunOptions, WslManager,
    },
};

pub struct AppArgs {
    pub config: Config,
    pub dry_run: bool,
    pub debug: bool,
    pub force: bool,
}

pub fn validate(config: Config) -> anyhow::Result<()> {
    let mut all_valid = true;
    for (name, instance) in &config.instances {
        match validate_instance(instance) {
            Ok(()) => eprintln!("{}", console::style(format!("✅ '{name}' is valid")).green()),
            Err(e) => {
                eprintln!("{}", console::style(format!("❌ '{name}': {e}")).red());
                all_valid = false;
            }
        }
    }
    if !all_valid {
        std::process::exit(1);
    }
    Ok(())
}

pub fn run(mut cfg: AppArgs) -> anyhow::Result<()> {
    ensure_windows()?;

    let manager: WslManager = WslManager::new(build_engine(EngineKind::Cli));
    let options: RunOptions = RunOptions {
        dry_run: cfg.dry_run,
        debug: cfg.debug,
    };

    for (instance_name, instance) in &cfg.config.instances {
        reporting::log_config_summary(instance_name, instance);
    }

    prompt_missing_cloud_init(&mut cfg.config, cfg.force)?;

    if !cfg.force {
        wizard::confirm_provision()?;
    }

    let results = manager.apply_all(&cfg.config, options)?;
    display::print_summary(&results);

    Ok(())
}

fn prompt_missing_cloud_init(config: &mut Config, force: bool) -> anyhow::Result<()> {
    let affected: Vec<&str> = config
        .instances
        .iter()
        .filter(|(_, instance)| !cloud_init_required(instance).is_empty())
        .map(|(name, _)| name.as_str())
        .collect();

    if affected.is_empty() {
        return Ok(());
    }

    for (name, instance) in config.instances.iter() {
        for reason in cloud_init_required(instance) {
            eprintln!("{}", console::style(format!("⚠️  '{name}': {reason}")).yellow());
        }
    }

    if force {
        return Ok(());
    }

    let choice = Select::new(
        "Enable default cloud-init for the above instance(s)?",
        vec!["yes, enable default cloud-init", "no, proceed without it"],
    )
    .with_help_message("esc to go back | ctrl+c to abort")
    .prompt()?;

    if choice == "yes, enable default cloud-init" {
        for instance in config.instances.values_mut() {
            if !cloud_init_required(instance).is_empty() {
                instance.default_cloud_init = true;
            }
        }
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
