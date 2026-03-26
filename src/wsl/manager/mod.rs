//! `WslManager` is the use-case orchestrator for WSL workflows.

use std::collections::BTreeMap;

use console::style;

use crate::config::{Config, ImageSource, Instance, SourcePath};
use crate::display;
use crate::wsl::engine::WslEngine;
use crate::wsl::setup;
use crate::wsl::validation::{environment, instance};
use crate::wsl::{Event, InstanceResult, RunOptions, Status};

mod steps;
use steps::{delete_instance, execute_create, execute_file_transfers, execute_scripts, prepare_provision};

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
        let instance_exists = self.engine.instance_exists(&instance.hostname)?;
        events.push(if instance_exists {
            Event::InstanceFound
        } else {
            Event::InstanceNotFound
        });

        let was_overridden = instance.override_instance && instance_exists;
        if instance.override_instance {
            events.push(Event::OverrideEnabled);
            events.extend(prepare_provision(self.engine.as_ref(), instance, options)?);
            events.extend(delete_instance(
                self.engine.as_ref(),
                &instance.hostname,
                instance_exists,
                options.dry_run,
            )?);
        } else if instance_exists {
            return Ok(InstanceResult {
                hostname: instance.hostname.clone(),
                outcome: Status::AlreadyExists,
                events,
            });
        } else {
            events.extend(prepare_provision(self.engine.as_ref(), instance, options)?);
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
        events.extend(execute_create(self.engine.as_ref(), instance)?);
        let outcome = if was_overridden {
            Status::Recreated
        } else {
            Status::Created
        };
        Ok(InstanceResult {
            hostname: instance.hostname.clone(),
            outcome,
            events,
        })
    }

    pub fn apply_instance(&self, instance: &Instance, options: RunOptions) -> anyhow::Result<InstanceResult> {
        let is_remote = matches!(
            &instance.image,
            ImageSource::File {
                path: SourcePath::Remote(_)
            }
        );
        let pb = is_remote.then(|| display::spinner("⬇️  Downloading image...".to_string()));
        let mut result = match self.create_instance(instance, options) {
            Ok(r) => r,
            Err(e) => return Ok(InstanceResult {
                hostname: instance.hostname.clone(),
                outcome: Status::Failed(e.to_string()),
                events: vec![],
            }),
        };
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }

        if matches!(result.outcome, Status::Created | Status::Recreated) && !instance.files.is_empty() {
            match display::with_spinner(format!("📂 Transferring {} file(s)...", instance.files.len()), || {
                execute_file_transfers(self.engine.as_ref(), instance)
            }) {
                Ok(events) => result.events.extend(events),
                Err(e) => result.outcome = Status::Failed(e.to_string()),
            }
        }

        if matches!(result.outcome, Status::Created | Status::Recreated) && !instance.scripts.run.is_empty() {
            match display::with_spinner(
                format!("⚙️  Running {} script(s)...", instance.scripts.run.len()),
                || execute_scripts(self.engine.as_ref(), instance),
            ) {
                Ok(events) => result.events.extend(events),
                Err(e) => result.outcome = Status::Failed(e.to_string()),
            }
        }

        result.log();
        Ok(result)
    }

    pub fn apply_all(
        &self,
        config: &Config,
        options: RunOptions,
    ) -> anyhow::Result<BTreeMap<String, InstanceResult>> {
        self.prepare_environment(options.dry_run)?;
        let mut results: BTreeMap<String, InstanceResult> = BTreeMap::new();
        for (instance_name, instance) in &config.instances {
            eprintln!("{}", style(format!("🔧 Creating '{instance_name}'...")).bold());
            results.insert(instance_name.clone(), self.apply_instance(instance, options)?);
        }
        Ok(results)
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/manager_tests.rs"]
mod manager_tests;
