use crate::config::{AppConfig, ImageSource};
use encoding_rs::UTF_16LE;
use log::{debug, info, warn};
use std::process::Command;

pub fn validate_all(cfg: &AppConfig) -> anyhow::Result<()> {
    validate_wsl_installed()?;
    update_wsl_version()?;
    validate_windows_features(&["Microsoft-Windows-Subsystem-Linux"])?;
    validate_image_source(cfg)?;
    Ok(())
}

pub fn validate_wsl_installed() -> anyhow::Result<()> {
    let output = Command::new("wsl.exe").arg("--status").output()?;
    if output.status.success() {
        info!("✅ WSL is installed");
        Ok(())
    } else {
        anyhow::bail!("⛔ WSL is not installed.")
    }
}

pub fn update_wsl_version() -> anyhow::Result<()> {
    let output = Command::new("wsl.exe").arg("--update").output()?;
    if output.status.success() {
        info!("✅ WSL update completed");
        Ok(())
    } else {
        anyhow::bail!("⛔ Failed to update WSL.")
    }
}

pub fn validate_image_source(cfg: &AppConfig) -> anyhow::Result<()> {
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

pub fn validate_windows_features(feature_names: &[&str]) -> anyhow::Result<()> {
    for feature_name in feature_names {
        match is_windows_feature_enabled(feature_name)? {
            true => info!("✅ {feature_name} is enabled"),
            false => warn!("⚠️  {feature_name} is not enabled"),
        }
    }
    Ok(())
}

fn is_likely_rootfs_archive(path: &std::path::Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    name.ends_with(".tar") || name.ends_with(".tar.gz") || name.ends_with(".tgz")
}

//
// OS interaction helpers
//

fn is_valid_wsl_distro_name(name: &str) -> anyhow::Result<bool> {
    let output = Command::new("wsl.exe")
        .args(["--list", "--online"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "wsl.exe --list --online failed with status {}",
            output.status
        );
    }

    let (text, _, _) = UTF_16LE.decode(&output.stdout);

    let ids: Vec<String> = text
        .lines()
        .map(str::trim)
        .skip_while(|l| !l.starts_with("NAME"))
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();

    debug!("Available WSL online distros: {:?}", ids);
    Ok(ids.iter().any(|id| id.eq_ignore_ascii_case(name)))
}

fn is_windows_feature_enabled(feature_name: &str) -> anyhow::Result<bool> {
    let output = Command::new("dism.exe")
        .args([
            "/English",
            "/online",
            "/Get-FeatureInfo",
            &format!("/featureName:{feature_name}"),
        ])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "dism.exe failed for feature '{feature_name}' with status {}\n{}",
            output.status,
            stdout.trim(),
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.trim() == "State : Enabled"))
}
