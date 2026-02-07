use crate::config::{ImageSource, Profile};
use crate::wsl::engine::CreateOutcome;
use crate::wsl::{cloud_init, provider, reporting, validation};
use log::info;

pub struct WslManager {
    provider: provider::WslProvider,
}

impl WslManager {
    pub fn new() -> Self {
        Self {
            provider: provider::WslProvider::new(provider::EngineKind::Cli),
        }
    }

    pub fn with_engine(kind: provider::EngineKind) -> Self {
        Self {
            provider: provider::WslProvider::new(kind),
        }
    }

    pub fn validate_environment(&self, dry_run: bool) -> anyhow::Result<()> {
        validation::validate_environment(dry_run)
    }

    pub fn create_instance(
        &self,
        profile_name: &str,
        profile: &Profile,
        dry_run: bool,
        debug: bool,
    ) -> anyhow::Result<()> {
        if !self.handle_override(profile, dry_run)? {
            reporting::log_create_outcome(
                CreateOutcome::AlreadyExists,
                &profile.hostname,
                matches!(profile.image, ImageSource::Distro { .. }),
            );
            return Ok(());
        }
        self.prepare_profile(profile, dry_run, debug)?;
        reporting::log_config_summary(profile_name, profile);
        if dry_run {
            info!("🧪 Dry run: WSL instance would be created");
            reporting::log_create_outcome(
                CreateOutcome::Skipped,
                &profile.hostname,
                matches!(profile.image, ImageSource::Distro { .. }),
            );
            return Ok(());
        }
        info!("🚀 Creating WSL instance");
        let outcome = self.create_profile(profile)?;
        reporting::log_create_outcome(
            outcome,
            &profile.hostname,
            matches!(profile.image, ImageSource::Distro { .. }),
        );
        Ok(())
    }

    fn handle_override(&self, profile: &Profile, dry_run: bool) -> anyhow::Result<bool> {
        let instance_exists = self.provider.instance_exists(&profile.hostname)?;
        if profile.override_instance {
            if !instance_exists {
                info!(
                    "ℹ️ WSL instance '{}' does not exist. Skipping delete.",
                    profile.hostname
                );
            } else if dry_run {
                info!(
                    "🧪 Dry run: WSL instance '{}' would be deleted before creation",
                    profile.hostname
                );
            } else {
                self.provider.delete_instance(&profile.hostname)?;
            }
            return Ok(true);
        }

        if instance_exists {
            return Ok(false);
        }
        Ok(true)
    }

    fn prepare_profile(&self, profile: &Profile, dry_run: bool, debug: bool) -> anyhow::Result<()> {
        validation::validate_image_source(profile)?;
        cloud_init::prepare_cloud_init(profile, dry_run, debug)?;
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
        Self::new()
    }
}
