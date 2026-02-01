use crate::config::{AppConfig, ImageSource};
use log::{debug, info, warn};
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
        self.validate_image_source(cfg)?;
        self.print_plan(cfg);
        Ok(())
    }

    pub fn create_instance(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        info!("🚀 Creating WSL instance");
        self.log_feature_state("Microsoft-Windows-Subsystem-Linux")?;
        self.validate_image_source(cfg)?;
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

    fn validate_image_source(&self, cfg: &AppConfig) -> anyhow::Result<()> {
        match &cfg.image {
            ImageSource::File { path } => {
                if !path.exists() {
                    anyhow::bail!("image file not found: {}", path.display());
                }
                if !is_likely_rootfs_archive(path) {
                    warn!(
                        "⚠️  Image file does not look like a rootfs archive (.tar/.tar.gz/.tgz): {}",
                        path.display()
                    );
                }
            }
            ImageSource::Distro { name } => {
                if !is_valid_wsl_distro_name(name)? {
                    anyhow::bail!("unknown WSL distro name: {name}");
                }
            }
        }
        Ok(())
    }
}

fn is_likely_rootfs_archive(path: &std::path::Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    name.ends_with(".tar") || name.ends_with(".tar.gz") || name.ends_with(".tgz")
}

#[cfg(target_os = "windows")]
fn is_valid_wsl_distro_name(name: &str) -> anyhow::Result<bool> {
    let output = Command::new("wsl.exe")
        .args(["--list", "--online"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("wsl.exe --list --online failed with status {}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let needle = name.trim().to_lowercase();

    for line in stdout.lines().skip(1) {
        println!("Checking line: {}", line);
        let candidate = line.trim();
        if candidate.is_empty() {
            continue;
        }
        if candidate.to_lowercase() == needle {
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(not(target_os = "windows"))]
fn is_valid_wsl_distro_name(_name: &str) -> anyhow::Result<bool> {
    anyhow::bail!("WSL distro validation is only supported on Windows")
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
            color_status(output.status),
            color_stdout(stdout.trim()),
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.trim() == "State : Enabled"))
}

#[cfg(not(target_os = "windows"))]
fn is_windows_feature_enabled(_feature_name: &str) -> anyhow::Result<bool> {
    anyhow::bail!("Windows feature checks are only supported on Windows")
}

fn color_status(status: std::process::ExitStatus) -> String {
    format!("{status}").red().to_string()
}

fn color_stdout(message: &str) -> String {
    message.yellow().to_string()
}
