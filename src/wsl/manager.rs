use crate::config::{ImageSource, Profile};
use crate::wsl::helpers::expand_env_vars;
use crate::wsl::{cloud_init, commands, validation};
use log::info;

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_instance(
        &self,
        profile_name: &str,
        profile: &Profile,
        dry_run: bool,
        debug: bool,
    ) -> anyhow::Result<()> {
        validation::validate_image_source(profile)?;
        cloud_init::prepare_cloud_init(profile, dry_run, debug)?;
        self.log_config_summary(profile_name, profile);
        if dry_run {
            info!("🧪 Dry run: WSL instance would be created");
        } else {
            info!("🚀 Creating WSL instance");
            commands::create_instance(profile)?;
        }
        Ok(())
    }

    fn log_config_summary(&self, profile_name: &str, profile: &Profile) {
        info!("🧩 Profile: {}", profile_name);
        info!("🏷️ Hostname: {}", profile.hostname);
        info!("👤 User: {}", profile.username);
        let expanded_install_dir = expand_env_vars(&profile.install_dir.to_string_lossy())
            .unwrap_or_else(|_| profile.install_dir.to_string_lossy().into_owned());
        info!("📦 Install dir: {}", expanded_install_dir);
        match &profile.cloud_init {
            Some(source) => info!("☁️ Cloud-init: {}", source),
            None => info!("☁️ Cloud-init: not configured"),
        }

        match &profile.image {
            ImageSource::Distro { name } => {
                info!("🐧 Using WSL distro '{}'", name);
            }
            ImageSource::File { path } => {
                info!("🗂️  Using image file {:?}", path);
            }
        }

        if let Some(proxy) = &profile.http_proxy {
            info!("🌐 HTTP proxy: {}", proxy);
        }
        if let Some(proxy) = &profile.https_proxy {
            info!("🔐 HTTPS proxy: {}", proxy);
        }
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new()
    }
}
