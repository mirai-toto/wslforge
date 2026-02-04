use crate::config::{AppConfig, ImageSource};
use crate::wsl::env::expand_env_vars;
use crate::wsl::{cloud_init, commands, validation};
use log::{debug, info};

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_instance(&self, cfg: &AppConfig, dry_run: bool) -> anyhow::Result<()> {
        validation::validate_all(cfg)?;
        self.print_plan(cfg);
        if dry_run {
            info!("🧪 Dry run: WSL instance would be created");
        } else {
            cloud_init::prepare_cloud_init(cfg)?;
            info!("🚀 Creating WSL instance");
            commands::create_instance(cfg)?;
        }
        Ok(())
    }

    fn print_plan(&self, cfg: &AppConfig) {
        debug!("🏷️ Hostname: {}", cfg.hostname);
        debug!("👤 User: {}", cfg.username);
        let expanded_install_dir = expand_env_vars(&cfg.install_dir.to_string_lossy())
            .unwrap_or_else(|_| cfg.install_dir.to_string_lossy().into_owned());
        debug!("📦 Install dir: {}", expanded_install_dir);
        match &cfg.cloud_init {
            Some(source) => debug!("☁️ Cloud-init: {}", source),
            None => debug!("☁️ Cloud-init: not configured"),
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
            debug!("🌐 HTTP proxy: {}", proxy);
        }
        if let Some(proxy) = &cfg.https_proxy {
            debug!("🔐 HTTPS proxy: {}", proxy);
        }
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new()
    }
}
