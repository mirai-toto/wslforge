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
            install_distro(name)
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

fn install_distro(distro_name: &str) -> anyhow::Result<()> {
    // TODO: WSL `--install -d` uses the distro name as the instance name.
    // If we need custom instance names, consider install + export + import flow.
    info!("🔍 Checking if WSL instance '{}' exists...", distro_name);
    let exists = Command::new("wsl.exe")
        .args(["-d", distro_name, "--", "echo", "Already exists."])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if exists {
        info!("ℹ️ WSL instance '{}' already exists.", distro_name);
        return Ok(());
    }

    info!("🚧 Instance not found. Installing '{}'...", distro_name);

    let mut cmd = Command::new("wsl.exe");
    cmd.args(["--install", "-d", distro_name]);

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

    info!("✅ WSL instance '{}' installed successfully.", distro_name);
    Ok(())
}
