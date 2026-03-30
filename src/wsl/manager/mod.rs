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
use steps::{create, prepare_instance, run_scripts, transfer_files, wait_for_provisioning, CreateDecision};

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
        let instance_exists = self.engine.instance_exists(&instance.name)?;
        events.push(if instance_exists {
            Event::InstanceFound
        } else {
            Event::InstanceNotFound
        });

        let decision: CreateDecision = prepare_instance(self.engine.as_ref(), instance, instance_exists, options)?;
        if decision.skip {
            return Ok(InstanceResult {
                name: instance.name.clone(),
                outcome: Status::AlreadyExists,
                events,
            });
        }
        events.extend(decision.events);
        let recreated = decision.recreated;

        if options.dry_run {
            events.push(Event::CreateDryRun);
            return Ok(InstanceResult {
                name: instance.name.clone(),
                outcome: Status::Skipped,
                events,
            });
        }

        events.push(Event::CreateStarted);
        events.extend(create(self.engine.as_ref(), instance)?);
        let outcome = if recreated { Status::Recreated } else { Status::Created };
        Ok(InstanceResult {
            name: instance.name.clone(),
            outcome,
            events,
        })
    }

    pub fn apply_instance(&self, instance: &Instance, options: RunOptions) -> anyhow::Result<InstanceResult> {
        let mut result = self.create_instance_with_progress(instance, options)?;

        if matches!(result.outcome, Status::Created | Status::Recreated) {
            let pb = display::spinner("⏳ Waiting for provisioning...".to_string());
            let provisioning_result = wait_for_provisioning(self.engine.as_ref(), instance, &|s| {
                pb.set_message(format!("⏳ Waiting for provisioning... {s}"))
            });
            pb.finish_and_clear();
            match provisioning_result {
                Ok(events) => result.events.extend(events),
                Err(e) => result.outcome = Status::Failed(e.to_string()),
            }
        }

        if !instance.files.is_empty() {
            self.run_step(
                &mut result,
                format!("📂 Transferring {} file(s)...", instance.files.len()),
                || transfer_files(self.engine.as_ref(), instance),
            );
        }
        if !instance.scripts.run.is_empty() {
            self.run_step(
                &mut result,
                format!("⚙️  Running {} script(s)...", instance.scripts.run.len()),
                || run_scripts(self.engine.as_ref(), instance),
            );
        }

        result.log();
        Ok(result)
    }

    /// Runs `create_instance`, wrapping it in a download spinner for remote images.
    fn create_instance_with_progress(
        &self,
        instance: &Instance,
        options: RunOptions,
    ) -> anyhow::Result<InstanceResult> {
        let is_remote = matches!(
            &instance.image,
            ImageSource::File {
                path: SourcePath::Remote(_)
            }
        );
        let pb = is_remote.then(|| display::spinner("⬇️Downloading image...".to_string()));
        let result = self
            .create_instance(instance, options)
            .unwrap_or_else(|e| InstanceResult {
                name: instance.name.clone(),
                outcome: Status::Failed(e.to_string()),
                events: vec![],
            });
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        Ok(result)
    }

    /// Runs a labelled step only when the instance was just created/recreated.
    /// On failure, transitions the outcome to `Status::Failed` instead of propagating.
    fn run_step<F>(&self, result: &mut InstanceResult, label: String, f: F)
    where
        F: FnOnce() -> anyhow::Result<Vec<Event>>,
    {
        if !matches!(result.outcome, Status::Created | Status::Recreated) {
            return;
        }
        match display::with_spinner(label, f) {
            Ok(events) => result.events.extend(events),
            Err(e) => result.outcome = Status::Failed(e.to_string()),
        }
    }

    pub fn apply_all(&self, config: &Config, options: RunOptions) -> anyhow::Result<BTreeMap<String, InstanceResult>> {
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
