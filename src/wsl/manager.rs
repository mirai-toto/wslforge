use crate::config::{AppConfig, ImageSource};
use crate::wsl::validation;
use log::{debug, info};

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_instance(&self, cfg: &AppConfig, dry_run: bool) -> anyhow::Result<()> {
        if dry_run {
            info!("🧪 Dry run: WSL instance would be created");
        } else {
            info!("🚀 Creating WSL instance");
        }
        validation::validate_all(cfg)?;
        self.print_plan(cfg);
        if dry_run {
            info!("🧩 Instance creation not implemented yet (mock)");
            return Ok(());
        }

        info!("🧩 Instance creation not implemented yet (mock)");
        Ok(())
    }

    fn print_plan(&self, cfg: &AppConfig) {
        debug!("🏷️ Hostname: {}", cfg.hostname);
        debug!("👤 User: {}", cfg.username);
        debug!("📦 Install dir: {:?}", cfg.install_dir);
        debug!("☁️ Cloud-init: {:?}", cfg.cloud_init);

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
