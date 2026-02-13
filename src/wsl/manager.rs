//! `WslManager` is the use-case orchestrator for WSL workflows.
//! It intentionally owns branching/sequencing decisions while delegating
//! low-level mechanics (CLI/process/fs specifics) to helpers/adapters.

use std::collections::BTreeMap;

use crate::config::{ImageSource, Profile, RootConfig};
use crate::wsl::engine::WslEngine;
use crate::wsl::maintenance;
use crate::wsl::validation::{config, environment};
use crate::wsl::{
    cloud_init, helpers::path, CreateEvent, CreateOutcome, CreateReport, EnvironmentReport, ExecutionOptions,
};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
}

impl WslManager {
    pub fn new(engine: Box<dyn WslEngine>) -> Self {
        Self { engine }
    }

    pub fn validate_environment(&self) -> anyhow::Result<EnvironmentReport> {
        let events = environment::check_environment()?;
        Ok(EnvironmentReport { events })
    }

    pub fn prepare_environment(&self, dry_run: bool) -> anyhow::Result<EnvironmentReport> {
        let mut report = self.validate_environment()?;
        let update_event = maintenance::environment::update_wsl_version(dry_run)?;
        report.events.push(update_event);
        Ok(report)
    }

    pub fn create_instance(&self, profile: &Profile, options: ExecutionOptions) -> anyhow::Result<CreateReport> {
        config::validate_profile(profile)?;

        let mut events = vec![CreateEvent::InstanceCheckStarted];
        let instance_exists = self.engine.instance_exists(&profile.hostname)?;
        if instance_exists {
            events.push(CreateEvent::InstanceExists);
        } else {
            events.push(CreateEvent::InstanceMissing);
        }

        if profile.override_instance {
            events.push(CreateEvent::OverrideRequested);
            self.prepare_profile(profile, options, &mut events)?;
            self.delete_instance(&profile.hostname, instance_exists, options, &mut events)?;
        } else if instance_exists {
            return Ok(CreateReport {
                outcome: CreateOutcome::AlreadyExists,
                events,
            });
        } else {
            self.prepare_profile(profile, options, &mut events)?;
        }

        if options.dry_run {
            events.push(CreateEvent::CreateDryRun);
            return Ok(CreateReport {
                outcome: CreateOutcome::Skipped,
                events,
            });
        }

        events.push(CreateEvent::CreateStarted);
        let outcome = self.create_profile(profile)?;
        Ok(CreateReport { outcome, events })
    }

    pub fn apply_config(
        &self,
        root: &RootConfig,
        options: ExecutionOptions,
    ) -> anyhow::Result<(EnvironmentReport, BTreeMap<String, CreateReport>)> {
        let environment_report = self.prepare_environment(options.dry_run)?;

        let mut create_reports_by_profile = BTreeMap::new();
        for (profile_name, profile) in &root.profiles {
            let report = self.create_instance(profile, options)?;
            create_reports_by_profile.insert(profile_name.clone(), report);
        }

        Ok((environment_report, create_reports_by_profile))
    }

    fn delete_instance(
        &self,
        hostname: &str,
        instance_exists: bool,
        options: ExecutionOptions,
        events: &mut Vec<CreateEvent>,
    ) -> anyhow::Result<()> {
        if !instance_exists {
            events.push(CreateEvent::DeleteSkippedMissing);
            return Ok(());
        }

        events.push(CreateEvent::OverrideExistingInstance);
        if options.dry_run {
            events.push(CreateEvent::DeleteDryRun);
            return Ok(());
        }

        events.push(CreateEvent::DeleteStarted);
        self.engine.delete_instance(hostname)?;
        events.push(CreateEvent::DeleteCompleted);
        Ok(())
    }

    fn prepare_profile(
        &self,
        profile: &Profile,
        options: ExecutionOptions,
        events: &mut Vec<CreateEvent>,
    ) -> anyhow::Result<()> {
        if let ImageSource::Distro { name } = &profile.image {
            environment::validate_wsl_distro_name(name)?;
        }

        let cloud_init_events = cloud_init::prepare_cloud_init(profile, options.dry_run, options.debug)?;
        events.extend(cloud_init_events.into_iter().map(CreateEvent::CloudInit));
        Ok(())
    }

    fn create_profile(&self, profile: &Profile) -> anyhow::Result<CreateOutcome> {
        match &profile.image {
            ImageSource::File { path: rootfs_tar } => {
                let install_dir = path::resolve_install_dir(&profile.install_dir, &profile.hostname)?;
                let rootfs_tar = path::expand_path(rootfs_tar.as_path())?;
                self.engine
                    .create_from_file(&profile.hostname, &install_dir, &rootfs_tar)?;
            }
            ImageSource::Distro { name } => {
                self.engine.create_from_distro(name, &profile.hostname)?;
            }
        }
        Ok(CreateOutcome::Created)
    }
}
