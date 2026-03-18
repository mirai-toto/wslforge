use crate::wsl::{Event, ProfileResult, Status};
use log::info;

pub fn log_create_report(report: &ProfileResult, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    let (icon, message) = match report.outcome {
        Status::Created => ("✅", format!("WSL instance '{}' created successfully.", hostname)),
        Status::AlreadyExists => ("ℹ️", format!("WSL instance '{}' already exists.", hostname)),
        Status::Skipped => ("ℹ️", format!("WSL instance '{}' was skipped.", hostname)),
    };
    info!("{} {}", icon, message);
}

fn log_create_event(event: &Event, hostname: &str) {
    match event {
        Event::InstanceCheckStarted => info!("🔍 Checking if WSL instance '{}' exists...", hostname),
        Event::InstanceFound => info!("✅ WSL instance '{}' exists.", hostname),
        Event::InstanceNotFound => info!("ℹ️ WSL instance '{}' does not exist.", hostname),
        Event::OverrideRequested => info!("⚠️ Override requested for WSL instance '{}'.", hostname),
        Event::OverrideStarted => {
            info!("⚠️ WSL instance '{}' already exists and will be overridden.", hostname)
        }
        Event::DeleteSkipped => {
            info!("ℹ️ WSL instance '{}' does not exist. Skipping delete.", hostname)
        }
        Event::DeleteDryRun => info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname),
        Event::DeleteStarted => info!("🧹 Deleting existing WSL instance '{}'", hostname),
        Event::DeleteCompleted => info!("✅ WSL instance '{}' deleted successfully.", hostname),
        Event::CreateDryRun => info!("🧪 Dry run: WSL instance '{}' would be created", hostname),
        Event::CreateStarted => info!("🚀 Creating WSL instance '{}'", hostname),
        Event::CloudInitSkipped => info!("☁️ Cloud-init: not configured"),
        Event::CloudInitSourceResolved(path) => info!("☁️ Cloud-init source: {}", path.display()),
        Event::CloudInitInlineLoaded => info!("☁️ Cloud-init source: inline content"),
        Event::CloudInitDryRunDeployed(path) => {
            info!("🧪 Dry run: cloud-init target would be created at: {}", path.display())
        }
        Event::CloudInitDeployed(path) => info!("☁️ Cloud-init target: {}", path.display()),
        Event::CloudInitDebugCopied(path) => info!("☁️ Cloud-init debug copy: {}", path.display()),
        Event::CloudInitDebugSkipped(reason) => info!("☁️ Cloud-init debug copy skipped ({reason})"),
    }
}
