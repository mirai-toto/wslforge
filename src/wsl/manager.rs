use crate::config::{AppConfig, ImageSource};
use log::{debug, info};

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn dry_run(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        info!("🧪 Dry run: WSL instance would be created");
        self.print_plan(cfg);
        Ok(())
    }

    pub fn create_instance(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        info!("🚀 Creating WSL instance");
        self.print_plan(cfg);
        info!("🧩 Instance creation not implemented yet (mock)");
        Ok(())
    }

    fn print_plan(&self, cfg: &AppConfig) {
        debug!("🏷️  Hostname: {}", cfg.hostname);
        debug!("👤 User: {}", cfg.username);
        debug!("📦 Install dir: {:?}", cfg.install_dir);
        debug!("☁️  Cloud-init: {:?}", cfg.cloud_init);

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
