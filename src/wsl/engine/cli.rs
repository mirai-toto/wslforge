use crate::wsl::engine::script::{copy_dir, copy_file, make_dirs, make_parent_dirs};
use crate::wsl::engine::{FileAttrs, WslEngine};
use crate::wsl::helpers::command_error;
use std::io::Write;
use std::path::Path;
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
        let output: std::process::Output = Command::new("wsl.exe")
            .args([
                "--import",
                name,
                &install_dir.to_string_lossy(),
                &rootfs_tar.to_string_lossy(),
                "--version",
                "2",
            ])
            .output()?;
        if !output.status.success() {
            return Err(command_error("wsl.exe --import failed", &output));
        }
        Ok(())
    }

    fn create_from_distro(&self, distro_name: &str, instance_name: &str) -> anyhow::Result<()> {
        let status: std::process::ExitStatus = Command::new("wsl.exe")
            .args(["--install", "-d", distro_name, "--name", instance_name, "--no-launch"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
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
        attrs: FileAttrs<'_>,
        shell: &str,
    ) -> anyhow::Result<()> {
        let FileAttrs { owner, group, mode } = attrs;
        let mut script = make_parent_dirs(dest, owner, group);
        script.extend(copy_file(dest, owner, group, mode));
        pipe_to_wsl(
            instance_name,
            shell,
            &script,
            content,
            &format!("failed to write '{dest}'"),
        )
    }

    fn write_dir(
        &self,
        instance_name: &str,
        src: &Path,
        dest: &str,
        attrs: FileAttrs<'_>,
        shell: &str,
    ) -> anyhow::Result<()> {
        let FileAttrs { owner, group, mode } = attrs;
        let mut tar_data: Vec<u8> = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_data);
            builder.append_dir_all(".", src)?;
            builder.finish()?;
        }
        let mut script = make_dirs(dest, owner, group);
        script.extend(copy_dir(dest, owner, group, mode));
        pipe_to_wsl(
            instance_name,
            shell,
            &script,
            &tar_data,
            &format!("failed to transfer directory to '{dest}'"),
        )
    }

    fn run_script(&self, instance_name: &str, script: &str, shell: &str) -> anyhow::Result<()> {
        let output: std::process::Output = Command::new("wsl.exe")
            .args(["-d", instance_name, "--", shell, "-c", script])
            .output()?;
        if !output.status.success() {
            return Err(command_error(&format!("script failed in '{instance_name}'"), &output));
        }
        Ok(())
    }

    fn wait_for_provisioning(&self, instance_name: &str, on_status: &dyn Fn(String)) -> anyhow::Result<String> {
        on_status("waiting...".to_string());

        let timeout = std::time::Duration::from_secs(300);
        let poll_interval = std::time::Duration::from_secs(2);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() >= timeout {
                anyhow::bail!(
                    "cloud-init timed out after {}s for '{}'",
                    timeout.as_secs(),
                    instance_name
                );
            }

            let output = Command::new("wsl.exe")
                .args(["-d", instance_name, "--", "cloud-init", "status"])
                .output()?;

            if !output.status.success() {
                log::debug!(
                    "cloud-init status exited non-zero for '{}': {}",
                    instance_name,
                    String::from_utf8_lossy(&output.stderr).trim()
                );
                return Ok(String::new());
            }

            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            on_status(stdout.clone());

            if stdout.contains("status: error") {
                anyhow::bail!("cloud-init failed for '{}': {}", instance_name, stdout);
            }
            if stdout.contains("status: done")
                || stdout.contains("status: disabled")
                || stdout.contains("status: not run")
            {
                return Ok(stdout);
            }

            std::thread::sleep(poll_interval);
        }
    }
}

fn pipe_to_wsl(
    instance_name: &str,
    shell: &str,
    script: &[String],
    data: &[u8],
    error_msg: &str,
) -> anyhow::Result<()> {
    let mut child = Command::new("wsl.exe")
        .args(["-d", instance_name, "--", shell, "-c", &script.join(" && ")])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child.stdin.take().expect("stdin configured").write_all(data)?;
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(command_error(error_msg, &output));
    }
    Ok(())
}
