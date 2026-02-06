use crate::config::{ImageSource, Profile};
use log::info;
use std::process::{Command, Stdio};

pub fn delete_instance_if_exists(instance_name: &str) -> anyhow::Result<()> {
    if !wsl_instance_exists(instance_name) {
        return Ok(());
    }

    info!("🧹 Deleting existing WSL instance '{}'", instance_name);
    let output = Command::new("wsl.exe")
        .args(["--unregister", instance_name])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "wsl.exe --unregister failed with status {}\n{}\n{}",
            output.status,
            stdout.trim(),
            stderr.trim()
        );
    }

    info!("✅ WSL instance '{}' deleted successfully.", instance_name);
    Ok(())
}

pub fn create_instance(profile: &Profile) -> anyhow::Result<()> {
    match &profile.image {
        ImageSource::File { path } => {
            let install_dir = profile.install_dir.join(&profile.hostname);
            info!(
                "📦 Creating WSL instance '{}' from rootfs file {}",
                profile.hostname,
                path.display()
            );
            import_rootfs(&profile.hostname, &install_dir, path)
        }
        ImageSource::Distro { name } => {
            info!("🐧 Installing WSL distro '{}'", name);
            install_distro(name, &profile.hostname)
        }
    }
}

fn import_rootfs(
    hostname: &str,
    install_dir: &std::path::Path,
    rootfs_tar: &std::path::Path,
) -> anyhow::Result<()> {
    if wsl_instance_exists(hostname) {
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
    if wsl_instance_exists(instance_name) {
        info!("ℹ️ WSL instance '{}' already exists.", instance_name);
        return Ok(());
    }

    info!(
        "🚧 Instance not found. Installing '{}' as '{}'...",
        distro_name, instance_name
    );

    let mut cmd = Command::new("wsl.exe");
    cmd.args(["--install", "-d", distro_name, "--name", instance_name]);

    let status = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("wsl.exe --install failed with status {}", status);
    }

    info!(
        "✅ WSL instance '{}' installed successfully.",
        instance_name
    );
    Ok(())
}

fn wsl_instance_exists(instance_name: &str) -> bool {
    info!("🔍 Checking if WSL instance '{}' exists...", instance_name);
    Command::new("wsl.exe")
        .args(["-d", instance_name, "--", "echo", "Already exists."])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
