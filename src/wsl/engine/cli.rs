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
        attrs: FileAttrs<'_>,
        shell: &str,
    ) -> anyhow::Result<()> {
        let FileAttrs { owner, group, mode } = attrs;
        let parent: String = std::path::Path::new(dest)
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();

        let mut script: Vec<String> = Vec::new();
        if !parent.is_empty() {
            script.push(install_dir(&parent, owner, group));
        }
        script.push(format!("cat > \"{dest}\""));
        if owner.is_some() || group.is_some() {
            script.push(chown_cmd(owner, group, dest));
        }
        if let Some(m) = mode {
            script.push(format!("chmod '{m}' \"{dest}\""));
        }

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
        let mut archive_buf: Vec<u8> = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut archive_buf);
            builder.append_dir_all(".", src)?;
            builder.finish()?;
        }

        let mut script: Vec<String> = vec![install_dir(dest, owner, group), format!("tar xf - -C \"{dest}\"")];
        if owner.is_some() || group.is_some() {
            script.push(format!("chown -R {} \"{dest}\"", chown_spec(owner, group)));
        }
        if let Some(m) = mode {
            script.push(format!("chmod -R '{m}' \"{dest}\""));
        } else {
            // tar archives built on Windows may not carry valid Unix file modes,
            // so apply sensible defaults: rwx for owner, r-x for group/others on
            // directories; rw for owner, r-- for group/others on regular files.
            script.push(format!("chmod -R 'u+rwX,go+rX' \"{dest}\""));
        }

        pipe_to_wsl(
            instance_name,
            shell,
            &script,
            &archive_buf,
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
}

/// Builds a `chown` ownership specifier from separate owner and group values.
fn chown_spec(owner: Option<&str>, group: Option<&str>) -> String {
    match (owner, group) {
        (Some(o), Some(g)) => format!("'{o}:{g}'"),
        (Some(o), None) => format!("'{o}'"),
        (None, Some(g)) => format!("':{g}'"),
        (None, None) => String::new(),
    }
}

/// Builds a `chown` command for a single path.
fn chown_cmd(owner: Option<&str>, group: Option<&str>, path: &str) -> String {
    format!("chown {} \"{path}\"", chown_spec(owner, group))
}

/// Builds an `install -d` command that creates a directory (and all intermediate
/// ancestors) with the specified owner and/or group. Falls back to `mkdir -p`
/// when neither is provided.
fn install_dir(path: &str, owner: Option<&str>, group: Option<&str>) -> String {
    if owner.is_none() && group.is_none() {
        return format!("mkdir -p \"{path}\"");
    }
    let mut cmd = String::from("install -d");
    if let Some(o) = owner {
        cmd.push_str(&format!(" -o '{o}'"));
    }
    if let Some(g) = group {
        cmd.push_str(&format!(" -g '{g}'"));
    }
    cmd.push_str(&format!(" \"{path}\""));
    cmd
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
