use crate::config::{AppConfig, ImageSource};
use crate::wsl::validation;
use log::{debug, info};
use owo_colors::OwoColorize;
use std::process::Command;

pub struct WslManager;

impl WslManager {
    pub fn new() -> Self {
        Self
    }

    pub fn dry_run(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        info!("🧪 Dry run: WSL instance would be created");
        self.log_feature_state("Microsoft-Windows-Subsystem-Linux")?;
        validation::validate_image_source(cfg)?;
        self.print_plan(cfg);
        Ok(())
    }

    pub fn create_instance(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        info!("🚀 Creating WSL instance");
        self.log_feature_state("Microsoft-Windows-Subsystem-Linux")?;
        validation::validate_image_source(cfg)?;
        self.print_plan(cfg);
        info!("🧩 Instance creation not implemented yet (mock)");
        Ok(())
    }

    fn log_feature_state(&self, feature_name: &str) -> anyhow::Result<()> {
        match is_windows_feature_enabled(feature_name)? {
            true => info!("✅ {feature_name} is enabled"),
            false => info!("⚠️  {feature_name} is not enabled"),
        }
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

#[cfg(target_os = "windows")]
fn is_windows_feature_enabled(feature_name: &str) -> anyhow::Result<bool> {
    let output = Command::new("dism.exe")
        .args([
            "/online",
            "/Get-FeatureInfo",
            &format!("/featureName:{feature_name}"),
        ])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "dism.exe failed for feature '{feature_name}' with status {}\n{}",
            format!("{}", output.status).red(),
            stdout.trim().yellow(),
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.trim() == "State : Enabled"))
}

#[cfg(not(target_os = "windows"))]
fn is_windows_feature_enabled(_feature_name: &str) -> anyhow::Result<bool> {
    anyhow::bail!("Windows feature checks are only supported on Windows")
}
