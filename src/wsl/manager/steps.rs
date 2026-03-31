use crate::config::{ImageSource, Instance, SourcePath};
use crate::wsl::cloud_init;
use crate::wsl::engine::{FileAttrs, WslEngine};
use crate::wsl::helpers::{expand_path, expand_wsl_dest, resolve_install_dir, resolve_source_path};
use crate::wsl::validation::environment;
use crate::wsl::{Event, RunOptions};

pub(super) const DEFAULT_SHELL: &str = "sh";

pub(super) struct CreateDecision {
    pub(super) skip: bool,
    pub(super) events: Vec<Event>,
    pub(super) recreated: bool,
}

pub(super) fn prepare_instance(
    engine: &dyn WslEngine,
    instance: &Instance,
    instance_exists: bool,
    options: RunOptions,
) -> anyhow::Result<CreateDecision> {
    if instance.override_instance {
        // Override mode: prepare provisioning then delete the existing instance
        // (delete_instance is a no-op when the instance does not exist yet).
        let mut events: Vec<Event> = vec![Event::OverrideEnabled];
        events.extend(setup_cloud_init(engine, instance, options)?);
        events.extend(delete_instance(
            engine,
            &instance.name,
            instance_exists,
            options.dry_run,
        )?);
        Ok(CreateDecision {
            skip: false,
            events,
            recreated: instance_exists,
        })
    } else if instance_exists {
        // No override and instance already exists: nothing to do.
        Ok(CreateDecision {
            skip: true,
            events: vec![],
            recreated: false,
        })
    } else {
        // Normal creation: prepare provisioning for a new instance.
        Ok(CreateDecision {
            skip: false,
            events: setup_cloud_init(engine, instance, options)?,
            recreated: false,
        })
    }
}

pub(super) fn setup_cloud_init(
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
    name: &str,
    instance_exists: bool,
    dry_run: bool,
) -> anyhow::Result<Vec<Event>> {
    if !instance_exists {
        return Ok(vec![Event::DeleteSkipped]);
    }
    if dry_run {
        return Ok(vec![Event::OverrideTriggered, Event::DeleteDryRun]);
    }
    engine.delete_instance(name)?;
    Ok(vec![
        Event::OverrideTriggered,
        Event::DeleteStarted,
        Event::DeleteCompleted,
    ])
}

pub(super) fn create(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let mut events: Vec<Event> = Vec::new();
    match &instance.image {
        ImageSource::File { path } => {
            let install_dir = resolve_install_dir(&instance.install_dir, &instance.name)?;
            if matches!(path, SourcePath::Remote(_)) {
                events.push(Event::ImageDownloadStarted);
            }
            let resolved = resolve_source_path(path)?;
            if matches!(path, SourcePath::Remote(_)) {
                events.push(Event::ImageDownloadCompleted);
            }
            engine.create_from_file(&instance.name, &install_dir, resolved.as_path())?;
        }
        ImageSource::Distro { name } => {
            engine.create_from_distro(name, &instance.name)?;
        }
    }
    Ok(events)
}

pub(super) fn transfer_files(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let shell = instance.scripts.shell.as_deref().unwrap_or(DEFAULT_SHELL);
    let mut events: Vec<Event> = Vec::new();
    let username = instance.username.as_deref().unwrap_or("");
    for transfer in &instance.files {
        let src = expand_path(&transfer.src)?;
        let dest = expand_wsl_dest(&transfer.dest, username, &src);
        let owner = transfer.owner.as_deref().or_else(|| {
            if dest.starts_with(&instance.user_home) {
                Some(username)
            } else {
                None
            }
        });
        // Default group to owner when not explicitly set.
        let group = transfer.group.as_deref().or(owner);
        if src.is_dir() {
            events.push(Event::DirectoryTransferStarted(src.clone()));
            engine.write_dir(
                &instance.name,
                &src,
                &dest,
                FileAttrs {
                    owner,
                    group,
                    mode: transfer.mode.as_deref(),
                },
                shell,
            )?;
            events.push(Event::DirectoryTransferCompleted(dest.clone()));
        } else {
            events.push(Event::FileTransferStarted(src.clone()));
            let content =
                std::fs::read(&src).map_err(|e| anyhow::anyhow!("failed to read '{}': {e}", src.display()))?;
            engine.write_file(
                &instance.name,
                &dest,
                &content,
                FileAttrs {
                    owner,
                    group,
                    mode: transfer.mode.as_deref(),
                },
                shell,
            )?;
            events.push(Event::FileTransferCompleted(dest.clone()));
        }
    }
    Ok(events)
}

pub(super) fn wait_for_provisioning(
    engine: &dyn WslEngine,
    instance: &Instance,
    on_status: &dyn Fn(String),
) -> anyhow::Result<(Vec<Event>, String)> {
    let cloud_init_active = instance.cloud_init.is_some() || instance.default_cloud_init;
    if !cloud_init_active {
        return Ok((vec![], String::new()));
    }
    let final_status = engine.wait_for_provisioning(&instance.name, on_status)?;
    Ok((
        vec![Event::ProvisioningWaiting, Event::ProvisioningCompleted],
        final_status,
    ))
}

pub(super) fn run_scripts(engine: &dyn WslEngine, instance: &Instance) -> anyhow::Result<Vec<Event>> {
    let shell = instance.scripts.shell.as_deref().unwrap_or(DEFAULT_SHELL);
    let mut events: Vec<Event> = Vec::new();
    for script in &instance.scripts.run {
        let expanded = script.replace("~/", &format!("{}/", instance.user_home));
        events.push(Event::ScriptStarted(script.clone()));
        engine.run_script(&instance.name, &expanded, shell)?;
        events.push(Event::ScriptCompleted(script.clone()));
    }
    Ok(events)
}
