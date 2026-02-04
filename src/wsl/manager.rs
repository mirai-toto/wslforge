use crate::config::{AppConfig, ImageSource};
use crate::wsl::env::expand_env_vars;
use crate::wsl::{cloud_init, commands, validation};
use log::info;

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_instance(&self, cfg: &AppConfig, dry_run: bool) -> anyhow::Result<()> {
        validation::validate_all(cfg)?;
        cloud_init::prepare_cloud_init(cfg)?;
        self.log_config_summary(cfg);
        if dry_run {
            info!("🧪 Dry run: WSL instance would be created");
        } else {
            info!("🚀 Creating WSL instance");
            commands::create_instance(cfg)?;
        }
        Ok(())
    }

    fn log_config_summary(&self, cfg: &AppConfig) {
        info!("🏷️ Hostname: {}", cfg.hostname);
        info!("👤 User: {}", cfg.username);
        let expanded_install_dir = expand_env_vars(&cfg.install_dir.to_string_lossy())
            .unwrap_or_else(|_| cfg.install_dir.to_string_lossy().into_owned());
        info!("📦 Install dir: {}", expanded_install_dir);
        match &cfg.cloud_init {
            Some(source) => info!("☁️ Cloud-init: {}", source),
            None => info!("☁️ Cloud-init: not configured"),
        }

        match &cfg.image {
            ImageSource::Distro { name } => {
                info!("🐧 Using WSL distro '{}'", name);
            }
            ImageSource::File { path } => {
                info!("🗂️  Using image file {:?}", path);
            }
        }

        if let Some(proxy) = &cfg.http_proxy {
            info!("🌐 HTTP proxy: {}", proxy);
        }
        if let Some(proxy) = &cfg.https_proxy {
            info!("🔐 HTTPS proxy: {}", proxy);
        }
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new()
    }
}
