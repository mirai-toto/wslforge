use crate::config::{ImageSource, Instance, SourcePath};
use crate::wsl::cloud_init;
use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::{expand_path, expand_wsl_dest, resolve_install_dir, resolve_source_path};
use crate::wsl::validation::environment;
use crate::wsl::{Event, RunOptions};

pub(super) const DEFAULT_SHELL: &str = "sh";

pub(super) fn prepare_provision(
    engine: &dyn WslEngine,
    instance: &Instance,
    options: RunOptions,
) -> anyhow::Result<Vec<Event>> {
    if let ImageSource::Distro { name } = &instance.image {
        environment::validate_wsl_distro_name(engine, name)?;
    }
    cloud_init::prepare_cloud_init(instance, options.dry_run, options.debug)
}

pub(super) fn delete_instance(
    engine: &dyn WslEngine,
    hostname: &str,
    instance_exists: bool,
    dry_run: bool,
) -> anyhow::Result<Vec<Event>> {
    if !instance_exists {
        return Ok(vec![Event::DeleteSkipped]);
    }
    if dry_run {
        return Ok(vec![Event::OverrideTriggered, Event::DeleteDryRun]);
    }
    engine.delete_instance(hostname)?;
    Ok(vec![
        Event::OverrideTriggered,
        Event::DeleteStarted,
        Event::DeleteCompleted,
    ])
}

pub(super) fn execute_create(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let mut events: Vec<Event> = Vec::new();
    match &instance.image {
        ImageSource::File { path } => {
            let install_dir = resolve_install_dir(&instance.install_dir, &instance.hostname)?;
            if matches!(path, SourcePath::Remote(_)) {
                events.push(Event::ImageDownloadStarted);
            }
            let resolved = resolve_source_path(path)?;
            if matches!(path, SourcePath::Remote(_)) {
                events.push(Event::ImageDownloadCompleted);
            }
            engine.create_from_file(&instance.hostname, &install_dir, resolved.as_path())?;
        }
        ImageSource::Distro { name } => {
            engine.create_from_distro(name, &instance.hostname)?;
        }
    }
    Ok(events)
}

pub(super) fn execute_file_transfers(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let shell = instance.scripts.shell.as_deref().unwrap_or(DEFAULT_SHELL);
    let mut events: Vec<Event> = Vec::new();
    for transfer in &instance.files {
        let src = expand_path(&transfer.src)?;
        let dest = expand_wsl_dest(&transfer.dest);
        if src.is_dir() {
            events.push(Event::DirectoryTransferStarted(src.clone()));
            engine.write_dir(
                &instance.hostname,
                &src,
                &dest,
                transfer.owner.as_deref(),
                transfer.mode.as_deref(),
                shell,
            )?;
            events.push(Event::DirectoryTransferCompleted(dest.clone()));
        } else {
            events.push(Event::FileTransferStarted(src.clone()));
            let content =
                std::fs::read(&src).map_err(|e| anyhow::anyhow!("failed to read '{}': {e}", src.display()))?;
            engine.write_file(
                &instance.hostname,
                &dest,
                &content,
                transfer.owner.as_deref(),
                transfer.mode.as_deref(),
                shell,
            )?;
            events.push(Event::FileTransferCompleted(dest.clone()));
        }
    }
    Ok(events)
}

pub(super) fn execute_scripts(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let shell = instance.scripts.shell.as_deref().unwrap_or(DEFAULT_SHELL);
    let mut events: Vec<Event> = Vec::new();
    for script in &instance.scripts.run {
        events.push(Event::ScriptStarted(script.clone()));
        engine.run_script(&instance.hostname, script, shell)?;
        events.push(Event::ScriptCompleted(script.clone()));
    }
    Ok(events)
}
