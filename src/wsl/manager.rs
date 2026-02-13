//! `WslManager` is the use-case orchestrator for WSL workflows.
//! It intentionally owns branching/sequencing decisions while delegating
//! low-level mechanics (CLI/process/fs specifics) to helpers/adapters.

use std::collections::BTreeMap;

use crate::config::{ImageSource, Profile, RootConfig};
use crate::wsl::engine::WslEngine;
use crate::wsl::maintenance;
use crate::wsl::validation::{config, environment};
use crate::wsl::{cloud_init, helpers::path, ExecutionOptions, Outcome, ProfileEvent, ProfileReport};

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
        maintenance::environment::update_wsl_version(self.engine.as_ref(), dry_run)?;
        Ok(())
    }

    pub fn create_instance(&self, profile: &Profile, options: ExecutionOptions) -> anyhow::Result<ProfileReport> {
        config::validate_profile(profile)?;

        let mut events = vec![ProfileEvent::InstanceCheckStarted];
        let instance_exists = self.engine.instance_exists(&profile.hostname)?;
        if instance_exists {
            events.push(ProfileEvent::InstanceExists);
        } else {
            events.push(ProfileEvent::InstanceMissing);
        }

        if profile.override_instance {
            events.push(ProfileEvent::OverrideRequested);
            self.prepare_profile(profile, options, &mut events)?;
            self.delete_instance(&profile.hostname, instance_exists, options, &mut events)?;
        } else if instance_exists {
            return Ok(ProfileReport {
                outcome: Outcome::AlreadyExists,
                events,
            });
        } else {
            self.prepare_profile(profile, options, &mut events)?;
        }

        if options.dry_run {
            events.push(ProfileEvent::CreateDryRun);
            return Ok(ProfileReport {
                outcome: Outcome::Skipped,
                events,
            });
        }

        events.push(ProfileEvent::CreateStarted);
        let outcome = self.create_profile(profile)?;
        Ok(ProfileReport { outcome, events })
    }

    pub fn apply_config(
        &self,
        root: &RootConfig,
        options: ExecutionOptions,
    ) -> anyhow::Result<BTreeMap<String, ProfileReport>> {
        self.prepare_environment(options.dry_run)?;

        let mut create_reports_by_profile = BTreeMap::new();
        for (profile_name, profile) in &root.profiles {
            let report = self.create_instance(profile, options)?;
            create_reports_by_profile.insert(profile_name.clone(), report);
        }

        Ok(create_reports_by_profile)
    }

    fn delete_instance(
        &self,
        hostname: &str,
        instance_exists: bool,
        options: ExecutionOptions,
        events: &mut Vec<ProfileEvent>,
    ) -> anyhow::Result<()> {
        if !instance_exists {
            events.push(ProfileEvent::DeleteSkippedMissing);
            return Ok(());
        }

        events.push(ProfileEvent::OverrideExistingInstance);
        if options.dry_run {
            events.push(ProfileEvent::DeleteDryRun);
            return Ok(());
        }

        events.push(ProfileEvent::DeleteStarted);
        self.engine.delete_instance(hostname)?;
        events.push(ProfileEvent::DeleteCompleted);
        Ok(())
    }

    fn prepare_profile(
        &self,
        profile: &Profile,
        options: ExecutionOptions,
        events: &mut Vec<ProfileEvent>,
    ) -> anyhow::Result<()> {
        if let ImageSource::Distro { name } = &profile.image {
            environment::validate_wsl_distro_name(self.engine.as_ref(), name)?;
        }

        let cloud_init_events = cloud_init::prepare_cloud_init(profile, options.dry_run, options.debug)?;
        events.extend(cloud_init_events);
        Ok(())
    }

    fn create_profile(&self, profile: &Profile) -> anyhow::Result<Outcome> {
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
        Ok(Outcome::Created)
    }
}
