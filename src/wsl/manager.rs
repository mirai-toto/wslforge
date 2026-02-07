use crate::config::{ImageSource, Profile};
use crate::wsl::engine::CreateOutcome;
use crate::wsl::{cloud_init, provider, reporting, validation};
use log::info;

pub struct WslManager {
    provider: provider::WslProvider,
    dry_run: bool,
    debug: bool,
}

impl WslManager {
    pub fn new(dry_run: bool, debug: bool) -> Self {
        Self {
            provider: provider::WslProvider::new(provider::EngineKind::Cli),
            dry_run,
            debug,
        }
    }

    pub fn with_engine(kind: provider::EngineKind, dry_run: bool, debug: bool) -> Self {
        Self {
            provider: provider::WslProvider::new(kind),
            dry_run,
            debug,
        }
    }

    pub fn validate_environment(&self) -> anyhow::Result<()> {
        validation::validate_environment(self.dry_run)
    }

    pub fn create_instance(&self, profile_name: &str, profile: &Profile) -> anyhow::Result<()> {
        let instance_exists = self.provider.instance_exists(&profile.hostname)?;
        if profile.override_instance {
            self.delete_instance(&profile.hostname, instance_exists)?;
        } else if instance_exists {
            reporting::log_create_outcome(CreateOutcome::AlreadyExists, &profile.hostname);
            return Ok(());
        }

        self.prepare_profile(profile)?;
        reporting::log_config_summary(profile_name, profile);

        if self.dry_run {
            info!("🧪 Dry run: WSL instance would be created");
            reporting::log_create_outcome(CreateOutcome::Skipped, &profile.hostname);
            return Ok(());
        }
        info!("🚀 Creating WSL instance");
        let outcome = self.create_profile(profile)?;
        reporting::log_create_outcome(outcome, &profile.hostname);
        Ok(())
    }

    fn delete_instance(&self, hostname: &str, instance_exists: bool) -> anyhow::Result<()> {
        if !instance_exists {
            info!(
                "ℹ️ WSL instance '{}' does not exist. Skipping delete.",
                hostname
            );
            return Ok(());
        } else {
            info!(
                "⚠️ WSL instance '{}' already exists and will be overridden.",
                hostname
            );
        }
        if self.dry_run {
            info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname);
            return Ok(());
        }
        self.provider.delete_instance(hostname)
    }

    fn prepare_profile(&self, profile: &Profile) -> anyhow::Result<()> {
        validation::validate_image_source(profile)?;
        cloud_init::prepare_cloud_init(profile, self.dry_run, self.debug)?;
        Ok(())
    }

    fn create_profile(&self, profile: &Profile) -> anyhow::Result<CreateOutcome> {
        match &profile.image {
            ImageSource::File { path: rootfs_tar } => {
                let install_dir = profile.install_dir.join(&profile.hostname);
                self.provider
                    .create_from_file(&profile.hostname, &install_dir, rootfs_tar)
            }
            ImageSource::Distro { name } => {
                self.provider.create_from_distro(name, &profile.hostname)
            }
        }
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new(false, false)
    }
}
