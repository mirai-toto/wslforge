use crate::wsl::engine::WslEngine;
use std::process::{Command, Stdio};

pub struct CliEngine;

impl CliEngine {
    pub fn new() -> Self {
        Self
    }
}

impl WslEngine for CliEngine {
    fn instance_exists(&self, name: &str) -> anyhow::Result<bool> {
        let status = Command::new("wsl.exe")
            .args(["-d", name, "--", "echo", "Already exists."])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    fn delete_instance(&self, name: &str) -> anyhow::Result<()> {
        let output = Command::new("wsl.exe").args(["--unregister", name]).output()?;

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
        Ok(())
    }

    fn create_from_file(
        &self,
        name: &str,
        install_dir: &std::path::Path,
        rootfs_tar: &std::path::Path,
    ) -> anyhow::Result<()> {
        let mut cmd = Command::new("wsl.exe");
        cmd.args([
            "--import",
            name,
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
        Ok(())
    }

    fn create_from_distro(&self, distro_name: &str, name: &str) -> anyhow::Result<()> {
        let mut cmd = Command::new("wsl.exe");
        cmd.args(["--install", "-d", distro_name, "--name", name, "--no-launch"]);

        let status = cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;
        if !status.success() {
            anyhow::bail!("wsl.exe --install failed with status {}", status);
        }
        Ok(())
    }
}
