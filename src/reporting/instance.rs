use crate::wsl::{Outcome, ProfileResult, ProvisionEvent};
use log::info;

pub fn log_create_report(report: &ProfileResult, hostname: &str) {
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

fn log_create_event(event: &ProvisionEvent, hostname: &str) {
    match event {
        ProvisionEvent::InstanceCheckStarted => info!("🔍 Checking if WSL instance '{}' exists...", hostname),
        ProvisionEvent::InstanceExists => info!("✅ WSL instance '{}' exists.", hostname),
        ProvisionEvent::InstanceMissing => info!("ℹ️ WSL instance '{}' does not exist.", hostname),
        ProvisionEvent::OverrideRequested => info!("⚠️ Override requested for WSL instance '{}'.", hostname),
        ProvisionEvent::OverrideExistingInstance => {
            info!("⚠️ WSL instance '{}' already exists and will be overridden.", hostname)
        }
        ProvisionEvent::DeleteSkippedMissing => {
            info!("ℹ️ WSL instance '{}' does not exist. Skipping delete.", hostname)
        }
        ProvisionEvent::DeleteDryRun => info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname),
        ProvisionEvent::DeleteStarted => info!("🧹 Deleting existing WSL instance '{}'", hostname),
        ProvisionEvent::DeleteCompleted => info!("✅ WSL instance '{}' deleted successfully.", hostname),
        ProvisionEvent::CreateDryRun => info!("🧪 Dry run: WSL instance '{}' would be created", hostname),
        ProvisionEvent::CreateStarted => info!("🚀 Creating WSL instance '{}'", hostname),
        ProvisionEvent::CloudInitNotConfigured => info!("☁️ Cloud-init: not configured"),
        ProvisionEvent::CloudInitSourceFile(path) => info!("☁️ Cloud-init source: {}", path.display()),
        ProvisionEvent::CloudInitSourceInline => info!("☁️ Cloud-init source: inline content"),
        ProvisionEvent::CloudInitDryRunTarget(path) => {
            info!("🧪 Dry run: cloud-init target would be created at: {}", path.display())
        }
        ProvisionEvent::CloudInitTargetWritten(path) => info!("☁️ Cloud-init target: {}", path.display()),
        ProvisionEvent::CloudInitDebugCopyWritten(path) => info!("☁️ Cloud-init debug copy: {}", path.display()),
        ProvisionEvent::CloudInitDebugCopySkipped(reason) => info!("☁️ Cloud-init debug copy skipped ({reason})"),
    }
}
