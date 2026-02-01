use crate::config::{AppConfig, ImageSource};
use log::warn;
use std::process::Command;

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
        anyhow::bail!(
            "wsl.exe --list --online failed with status {}",
            output.status
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let needle = name.trim().to_lowercase();

    for line in stdout.lines().skip(1) {
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
