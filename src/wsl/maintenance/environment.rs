use crate::wsl::EnvironmentEvent;
use std::process::Command;

pub fn update_wsl_version(dry_run: bool) -> anyhow::Result<EnvironmentEvent> {
    if dry_run {
        return Ok(EnvironmentEvent::WslUpdateDryRun);
    }
    let output = Command::new("wsl.exe").arg("--update").output()?;
    if output.status.success() {
        Ok(EnvironmentEvent::WslUpdateCompleted)
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to update WSL.\n{}\n{}", stdout.trim(), stderr.trim())
    }
}
