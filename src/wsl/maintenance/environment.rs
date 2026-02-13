use log::info;
use std::process::Command;

pub fn update_wsl_version(dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        info!("🧪 Dry run: WSL update would be performed");
        return Ok(());
    }
    let output = Command::new("wsl.exe").arg("--update").output()?;
    if output.status.success() {
        info!("✅ WSL update completed");
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to update WSL.\n{}\n{}", stdout.trim(), stderr.trim())
    }
}
