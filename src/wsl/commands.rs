use crate::config::{AppConfig, ImageSource};
use log::info;
use std::process::Command;

pub fn create_instance(cfg: &AppConfig) -> anyhow::Result<()> {
    match &cfg.image {
        ImageSource::File { path } => {
            let install_dir = cfg.install_dir.join(&cfg.hostname);
            info!(
                "📦 Creating WSL instance '{}' from rootfs file {}",
                cfg.hostname,
                path.display()
            );
            import_rootfs(&cfg.hostname, &install_dir, path)
        }
        ImageSource::Distro { name } => {
            info!("🐧 Installing WSL distro '{}'", name);
            install_distro(name, &cfg.hostname)
        }
    }
}

fn import_rootfs(
    hostname: &str,
    install_dir: &std::path::Path,
    rootfs_tar: &std::path::Path,
) -> anyhow::Result<()> {
    info!("🔍 Checking if WSL instance '{}' exists...", hostname);
    let exists = Command::new("wsl.exe")
        .args(["-d", hostname, "--", "echo", "Already exists."])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if exists {
        info!("ℹ️ WSL instance '{}' already exists.", hostname);
        return Ok(());
    }

    info!("🚧 Instance not found. Creating '{}'...", hostname);

    let mut cmd = Command::new("wsl.exe");
    cmd.args([
        "--import",
        hostname,
        &install_dir.to_string_lossy(),
        &rootfs_tar.to_string_lossy(),
        "--version",
        "2",
    ]);

    let output = cmd.output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "wsl.exe --import failed with status {}\n{}\n{}",
            output.status,
            stdout.trim(),
            stderr.trim()
        );
    }

    info!("✅ WSL instance '{}' created successfully.", hostname);
    Ok(())
}

fn install_distro(distro_name: &str, instance_name: &str) -> anyhow::Result<()> {
    info!("🔍 Checking if WSL instance '{}' exists...", instance_name);
    let exists = Command::new("wsl.exe")
        .args(["-d", instance_name, "--", "echo", "Already exists."])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if exists {
        info!("ℹ️ WSL instance '{}' already exists.", instance_name);
        return Ok(());
    }

    info!(
        "🚧 Instance not found. Installing '{}' as '{}'...",
        distro_name, instance_name
    );

    let mut cmd = Command::new("wsl.exe");
    cmd.args(["--install", "-d", distro_name, "--name", instance_name]);

    let output = cmd.output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "wsl.exe --install failed with status {}\n{}\n{}",
            output.status,
            stdout.trim(),
            stderr.trim()
        );
    }

    info!(
        "✅ WSL instance '{}' installed successfully.",
        instance_name
    );
    Ok(())
}
