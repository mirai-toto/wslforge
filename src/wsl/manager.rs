//! `WslManager` is the use-case orchestrator for WSL workflows.
//! It intentionally owns branching/sequencing decisions while delegating
//! low-level mechanics (CLI/process/fs specifics) to helpers/adapters.

use std::collections::BTreeMap;

use crate::config::{Config, ImageSource, Instance, SourcePath};
use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::{expand_path, resolve_install_dir};
use crate::wsl::setup;
use crate::wsl::validation::{environment, instance};
use crate::wsl::{cloud_init, Event, InstanceResult, RunOptions, Status};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
}

impl WslManager {
    pub fn new(engine: Box<dyn WslEngine>) -> Self {
        Self { engine }
    }

    pub fn validate_environment(&self) -> anyhow::Result<()> {
        environment::check_environment(self.engine.as_ref())
    }

    pub fn prepare_environment(&self, dry_run: bool) -> anyhow::Result<()> {
        self.validate_environment()?;
        setup::environment::update_wsl_version(self.engine.as_ref(), dry_run)?;
        Ok(())
    }

    pub fn create_instance(&self, instance: &Instance, options: RunOptions) -> anyhow::Result<InstanceResult> {
        instance::validate_instance(instance)?;

        let mut events: Vec<Event> = vec![Event::InstanceCheckStarted];
        let instance_exists: bool = self.engine.instance_exists(&instance.hostname)?;
        if instance_exists {
            events.push(Event::InstanceFound);
        } else {
            events.push(Event::InstanceNotFound);
        }

        if instance.override_instance {
            events.push(Event::OverrideEnabled);
            events.extend(self.prepare_provision(instance, options)?);
            events.extend(self.delete_instance(&instance.hostname, instance_exists, options)?);
        } else if instance_exists {
            return Ok(InstanceResult {
                hostname: instance.hostname.clone(),
                outcome: Status::AlreadyExists,
                events,
            });
        } else {
            events.extend(self.prepare_provision(instance, options)?);
        }

        if options.dry_run {
            events.push(Event::CreateDryRun);
            return Ok(InstanceResult {
                hostname: instance.hostname.clone(),
                outcome: Status::Skipped,
                events,
            });
        }

        events.push(Event::CreateStarted);
        let outcome: Status = self.execute_create(instance)?;
        Ok(InstanceResult {
            hostname: instance.hostname.clone(),
            outcome,
            events,
        })
    }

    pub fn transfer_files(&self, instance: &Instance) -> anyhow::Result<Vec<Event>> {
        self.execute_file_transfers(instance)
    }

    pub fn run_scripts(&self, instance: &Instance) -> anyhow::Result<Vec<Event>> {
        self.execute_scripts(instance)
    }

    pub fn apply_config(&self, root: &Config, options: RunOptions) -> anyhow::Result<BTreeMap<String, InstanceResult>> {
        self.prepare_environment(options.dry_run)?;

        let mut results: BTreeMap<String, InstanceResult> = BTreeMap::new();
        for (instance_name, instance) in &root.instances {
            let result: InstanceResult = self.create_instance(instance, options)?;
            results.insert(instance_name.clone(), result);
        }

        Ok(results)
    }

    fn delete_instance(
        &self,
        hostname: &str,
        instance_exists: bool,
        options: RunOptions,
    ) -> anyhow::Result<Vec<Event>> {
        if !instance_exists {
            return Ok(vec![Event::DeleteSkipped]);
        }

        if options.dry_run {
            return Ok(vec![Event::OverrideTriggered, Event::DeleteDryRun]);
        }

        self.engine.delete_instance(hostname)?;
        Ok(vec![
            Event::OverrideTriggered,
            Event::DeleteStarted,
            Event::DeleteCompleted,
        ])
    }

    fn prepare_provision(&self, instance: &Instance, options: RunOptions) -> anyhow::Result<Vec<Event>> {
        if let ImageSource::Distro { name } = &instance.image {
            environment::validate_wsl_distro_name(self.engine.as_ref(), name)?;
        }

        cloud_init::prepare_cloud_init(instance, options.dry_run, options.debug)
    }

    fn execute_file_transfers(&self, instance: &Instance) -> anyhow::Result<Vec<Event>> {
        let mut events: Vec<Event> = Vec::new();
        for transfer in &instance.files {
            let src: std::path::PathBuf = expand_path(&transfer.src)?;
            events.push(Event::FileTransferStarted(src.clone()));
            let content: Vec<u8> =
                std::fs::read(&src).map_err(|e| anyhow::anyhow!("failed to read '{}': {e}", src.display()))?;
            self.engine.write_file(
                &instance.hostname,
                &transfer.dest,
                &content,
                transfer.owner.as_deref(),
                transfer.mode.as_deref(),
            )?;
            events.push(Event::FileTransferCompleted(transfer.dest.clone()));
        }
        Ok(events)
    }

    fn execute_scripts(&self, instance: &Instance) -> anyhow::Result<Vec<Event>> {
        let mut events: Vec<Event> = Vec::new();
        for script in &instance.scripts {
            events.push(Event::ScriptStarted(script.clone()));
            self.engine.run_script(&instance.hostname, script)?;
            events.push(Event::ScriptCompleted(script.clone()));
        }
        Ok(events)
    }

    fn execute_create(&self, instance: &Instance) -> anyhow::Result<Status> {
        match &instance.image {
            ImageSource::File { path } => {
                let install_dir: std::path::PathBuf = resolve_install_dir(&instance.install_dir, &instance.hostname)?;
                let rootfs_tar: std::path::PathBuf = match path {
                    SourcePath::Local(p) => expand_path(p)?,
                    SourcePath::Remote(url) => {
                        let response = ureq::get(url.as_str())
                            .call()
                            .map_err(|e| anyhow::anyhow!("failed to download '{}': {e}", url))?;
                        let mut tmp = tempfile::NamedTempFile::new()?;
                        std::io::copy(&mut response.into_reader(), tmp.as_file_mut())?;
                        let path = tmp.path().to_path_buf();
                        self.engine.create_from_file(&instance.hostname, &install_dir, &path)?;
                        return Ok(Status::Created);
                    }
                };
                self.engine
                    .create_from_file(&instance.hostname, &install_dir, &rootfs_tar)?;
            }
            ImageSource::Distro { name } => {
                self.engine.create_from_distro(name, &instance.hostname)?;
            }
        }
        Ok(Status::Created)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/wsl/manager_tests.rs"]
mod manager_tests;
