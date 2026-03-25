use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::command_error;
pub fn update_wsl_version(engine: &dyn WslEngine, dry_run: bool) -> anyhow::Result<()> {
    if dry_run {
        eprintln!("🧪 Dry run: WSL update would be performed");
        return Ok(());
    }
    let output: std::process::Output = engine.update()?;
    if output.status.success() {
        eprintln!("✅ WSL update completed");
        Ok(())
    } else {
        Err(command_error("Failed to update WSL", &output))
    }
}
