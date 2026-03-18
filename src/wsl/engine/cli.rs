use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::command_error;
use std::process::{Command, Stdio};

#[derive(Default)]
pub struct CliEngine;

impl WslEngine for CliEngine {
    fn status(&self) -> anyhow::Result<std::process::Output> {
        Ok(Command::new("wsl.exe").arg("--status").output()?)
    }

    fn update(&self) -> anyhow::Result<std::process::Output> {
        Ok(Command::new("wsl.exe").arg("--update").output()?)
    }

    fn list_online_distros(&self) -> anyhow::Result<std::process::Output> {
        Ok(Command::new("wsl.exe").args(["--list", "--online"]).output()?)
    }

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
            return Err(command_error("wsl.exe --unregister failed", &output));
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
            return Err(command_error("wsl.exe --import failed", &output));
        }
        Ok(())
    }

    fn create_from_distro(&self, distro_name: &str, instance_name: &str) -> anyhow::Result<()> {
        let mut cmd = Command::new("wsl.exe");
        cmd.args(["--install", "-d", distro_name, "--name", instance_name, "--no-launch"]);

        let status = cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;
        if !status.success() {
            anyhow::bail!("wsl.exe --install failed with status {}", status);
        }
        Ok(())
    }
}
