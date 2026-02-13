use crate::wsl::{Outcome, ProfileEvent, ProfileReport};
use log::info;

pub fn log_create_report(report: &ProfileReport, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    let (icon, message) = match report.outcome {
        Outcome::Created => ("✅", format!("WSL instance '{}' created successfully.", hostname)),
        Outcome::AlreadyExists => ("ℹ️", format!("WSL instance '{}' already exists.", hostname)),
        Outcome::Skipped => ("ℹ️", format!("WSL instance '{}' was skipped.", hostname)),
    };
    info!("{} {}", icon, message);
}

fn log_create_event(event: &ProfileEvent, hostname: &str) {
    match event {
        ProfileEvent::InstanceCheckStarted => info!("🔍 Checking if WSL instance '{}' exists...", hostname),
        ProfileEvent::InstanceExists => info!("✅ WSL instance '{}' exists.", hostname),
        ProfileEvent::InstanceMissing => info!("ℹ️ WSL instance '{}' does not exist.", hostname),
        ProfileEvent::OverrideRequested => info!("⚠️ Override requested for WSL instance '{}'.", hostname),
        ProfileEvent::OverrideExistingInstance => {
            info!("⚠️ WSL instance '{}' already exists and will be overridden.", hostname)
        }
        ProfileEvent::DeleteSkippedMissing => info!("ℹ️ WSL instance '{}' does not exist. Skipping delete.", hostname),
        ProfileEvent::DeleteDryRun => info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname),
        ProfileEvent::DeleteStarted => info!("🧹 Deleting existing WSL instance '{}'", hostname),
        ProfileEvent::DeleteCompleted => info!("✅ WSL instance '{}' deleted successfully.", hostname),
        ProfileEvent::CreateDryRun => info!("🧪 Dry run: WSL instance '{}' would be created", hostname),
        ProfileEvent::CreateStarted => info!("🚀 Creating WSL instance '{}'", hostname),
        ProfileEvent::CloudInitNotConfigured => info!("☁️ Cloud-init: not configured"),
        ProfileEvent::CloudInitSourceFile(path) => info!("☁️ Cloud-init source: {}", path.display()),
        ProfileEvent::CloudInitSourceInline => info!("☁️ Cloud-init source: inline content"),
        ProfileEvent::CloudInitDryRunTarget(path) => {
            info!("🧪 Dry run: cloud-init target would be created at: {}", path.display())
        }
        ProfileEvent::CloudInitTargetWritten(path) => info!("☁️ Cloud-init target: {}", path.display()),
        ProfileEvent::CloudInitDebugCopyWritten(path) => info!("☁️ Cloud-init debug copy: {}", path.display()),
        ProfileEvent::CloudInitDebugCopySkipped(reason) => info!("☁️ Cloud-init debug copy skipped ({reason})"),
    }
}
