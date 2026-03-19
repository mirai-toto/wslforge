use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::command_error;
use log::info;

pub fn update_wsl_version(engine: &dyn WslEngine, dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        info!("🧪 Dry run: WSL update would be performed");
        return Ok(());
    }
    let output: std::process::Output = engine.update()?;
    if output.status.success() {
        info!("✅ WSL update completed");
        Ok(())
    } else {
        Err(command_error("Failed to update WSL", &output))
    }
}
