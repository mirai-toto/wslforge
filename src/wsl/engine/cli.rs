use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::command_error;
use std::io::Write;
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
        let status: std::process::ExitStatus = Command::new("wsl.exe")
            .args(["-d", name, "--", "echo", "Already exists."])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    fn delete_instance(&self, name: &str) -> anyhow::Result<()> {
        let output: std::process::Output = Command::new("wsl.exe").args(["--unregister", name]).output()?;

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
        let mut cmd: Command = Command::new("wsl.exe");
        cmd.args([
            "--import",
            name,
            &install_dir.to_string_lossy(),
            &rootfs_tar.to_string_lossy(),
            "--version",
            "2",
        ]);

        let output: std::process::Output = cmd.output()?;
        if !output.status.success() {
            return Err(command_error("wsl.exe --import failed", &output));
        }
        Ok(())
    }

    fn create_from_distro(&self, distro_name: &str, instance_name: &str) -> anyhow::Result<()> {
        let mut cmd: Command = Command::new("wsl.exe");
        cmd.args(["--install", "-d", distro_name, "--name", instance_name, "--no-launch"]);

        let status: std::process::ExitStatus = cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).status()?;
        if !status.success() {
            anyhow::bail!("wsl.exe --install failed with status {}", status);
        }
        Ok(())
    }

    fn write_file(
        &self,
        instance_name: &str,
        dest: &str,
        content: &[u8],
        owner: Option<&str>,
        mode: Option<&str>,
    ) -> anyhow::Result<()> {
        let parent: String = std::path::Path::new(dest)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut script: Vec<String> = Vec::new();
        if !parent.is_empty() {
            script.push(format!("mkdir -p '{parent}'"));
        }
        script.push(format!("cat > '{dest}'"));
        if let Some(o) = owner {
            script.push(format!("chown '{o}' '{dest}'"));
        }
        if let Some(m) = mode {
            script.push(format!("chmod '{m}' '{dest}'"));
        }

        let mut child = Command::new("wsl.exe")
            .args(["-d", instance_name, "--", "bash", "-c", &script.join(" && ")])
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        child.stdin.take().expect("stdin configured").write_all(content)?;

        let output: std::process::Output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(command_error(&format!("failed to write '{dest}'"), &output));
        }
        Ok(())
    }

    fn run_script(&self, instance_name: &str, script: &str) -> anyhow::Result<()> {
        let output: std::process::Output = Command::new("wsl.exe")
            .args(["-d", instance_name, "--", "bash", "-c", script])
            .output()?;
        if !output.status.success() {
            return Err(command_error(&format!("script failed in '{instance_name}'"), &output));
        }
        Ok(())
    }
}
