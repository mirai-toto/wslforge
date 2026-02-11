use crate::config::{ImageSource, Profile};
use crate::wsl::engine::api::ApiEngine;
use crate::wsl::engine::cli::CliEngine;
use crate::wsl::engine::{EngineKind, WslEngine};
use crate::wsl::validation::{config, environment};
use crate::wsl::{cloud_init, helpers::path, CreateEvent, CreateOutcome, CreateReport, EnvironmentReport};
use std::path::{Path, PathBuf};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
    dry_run: bool,
    debug: bool,
}

impl WslManager {
    pub fn new(dry_run: bool, debug: bool) -> Self {
        Self {
            engine: build_engine(EngineKind::Cli),
            dry_run,
            debug,
        }
    }

    pub fn with_engine(kind: EngineKind, dry_run: bool, debug: bool) -> Self {
        Self {
            engine: build_engine(kind),
            dry_run,
            debug,
        }
    }

    pub fn validate_environment(&self) -> anyhow::Result<EnvironmentReport> {
        environment::validate_environment(self.dry_run)
    }

    pub fn validate_profile_config(&self, profile: &Profile) -> anyhow::Result<()> {
        config::validate_profile(profile)
    }

    pub fn create_instance(&self, _profile_name: &str, profile: &Profile) -> anyhow::Result<CreateReport> {
        self.validate_profile_config(profile)?;

        let mut events = vec![CreateEvent::InstanceCheckStarted];
        let instance_exists = self.engine.instance_exists(&profile.hostname)?;
        if instance_exists {
            events.push(CreateEvent::InstanceExists);
        } else {
            events.push(CreateEvent::InstanceMissing);
        }

        if profile.override_instance {
            events.push(CreateEvent::OverrideRequested);
            self.prepare_profile(profile, &mut events)?;
            self.delete_instance(&profile.hostname, instance_exists, &mut events)?;
        } else if instance_exists {
            return Ok(CreateReport {
                outcome: CreateOutcome::AlreadyExists,
                events,
            });
        } else {
            self.prepare_profile(profile, &mut events)?;
        }
        if self.dry_run {
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

    fn delete_instance(
        &self,
        hostname: &str,
        instance_exists: bool,
        events: &mut Vec<CreateEvent>,
    ) -> anyhow::Result<()> {
        if !instance_exists {
            events.push(CreateEvent::DeleteSkippedMissing);
            return Ok(());
        }
        events.push(CreateEvent::OverrideExistingInstance);

        if self.dry_run {
            events.push(CreateEvent::DeleteDryRun);
            return Ok(());
        }

        events.push(CreateEvent::DeleteStarted);
        self.engine.delete_instance(hostname)?;
        events.push(CreateEvent::DeleteCompleted);
        Ok(())
    }

    fn prepare_profile(&self, profile: &Profile, events: &mut Vec<CreateEvent>) -> anyhow::Result<()> {
        if let ImageSource::Distro { name } = &profile.image {
            environment::validate_wsl_distro_name(name)?;
        }
        let cloud_init_events = cloud_init::prepare_cloud_init(profile, self.dry_run, self.debug)?;
        events.extend(cloud_init_events.into_iter().map(CreateEvent::CloudInit));
        Ok(())
    }

    fn create_profile(&self, profile: &Profile) -> anyhow::Result<CreateOutcome> {
        match &profile.image {
            ImageSource::File { path: rootfs_tar } => {
                let install_dir = resolve_install_dir(profile)?;
                let rootfs_tar = resolve_rootfs_path(rootfs_tar.as_path())?;
                self.engine
                    .create_from_file(&profile.hostname, &install_dir, &rootfs_tar)?;
                Ok(CreateOutcome::Created)
            }
            ImageSource::Distro { name } => {
                self.engine.create_from_distro(name, &profile.hostname)?;
                Ok(CreateOutcome::Created)
            }
        }
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new(false, false)
    }
}

fn resolve_install_dir(profile: &Profile) -> anyhow::Result<PathBuf> {
    let expanded = path::expand_path(&profile.install_dir)?;
    Ok(expanded.join(&profile.hostname))
}

fn resolve_rootfs_path(rootfs_tar: &Path) -> anyhow::Result<PathBuf> {
    path::expand_path(rootfs_tar)
}

fn build_engine(kind: EngineKind) -> Box<dyn WslEngine> {
    match kind {
        EngineKind::Cli => Box::new(CliEngine::new()),
        EngineKind::Api => Box::new(ApiEngine::new()),
    }
}
