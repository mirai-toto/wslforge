use crate::wsl::{CloudInitEvent, CreateEvent, CreateOutcome, CreateReport};
use log::info;

pub fn log_create_report(report: &CreateReport, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    let (icon, message) = match report.outcome {
        CreateOutcome::Created => ("✅", format!("WSL instance '{}' created successfully.", hostname)),
        CreateOutcome::AlreadyExists => ("ℹ️", format!("WSL instance '{}' already exists.", hostname)),
        CreateOutcome::Skipped => ("ℹ️", format!("WSL instance '{}' was skipped.", hostname)),
    };
    info!("{} {}", icon, message);
}

fn log_create_event(event: &CreateEvent, hostname: &str) {
    match event {
        CreateEvent::InstanceCheckStarted => {
            info!("🔍 Checking if WSL instance '{}' exists...", hostname);
        }
        CreateEvent::InstanceExists => {
            info!("✅ WSL instance '{}' exists.", hostname);
        }
        CreateEvent::InstanceMissing => {
            info!("ℹ️ WSL instance '{}' does not exist.", hostname);
        }
        CreateEvent::OverrideRequested => {
            // Kept intentionally silent to preserve previous behavior.
        }
        CreateEvent::OverrideExistingInstance => {
            info!("⚠️ WSL instance '{}' already exists and will be overridden.", hostname);
        }
        CreateEvent::DeleteSkippedMissing => {
            info!("ℹ️ WSL instance '{}' does not exist. Skipping delete.", hostname);
        }
        CreateEvent::DeleteDryRun => {
            info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname);
        }
        CreateEvent::DeleteStarted => {
            info!("🧹 Deleting existing WSL instance '{}'", hostname);
        }
        CreateEvent::DeleteCompleted => {
            info!("✅ WSL instance '{}' deleted successfully.", hostname);
        }
        CreateEvent::CreateDryRun => {
            info!("🧪 Dry run: WSL instance would be created");
        }
        CreateEvent::CreateStarted => {
            info!("🚀 Creating WSL instance");
        }
        CreateEvent::CloudInit(cloud_init_event) => log_cloud_init_event(cloud_init_event),
    }
}

fn log_cloud_init_event(event: &CloudInitEvent) {
    match event {
        CloudInitEvent::NotConfigured => info!("☁️ Cloud-init: not configured"),
        CloudInitEvent::SourceFile(path) => info!("☁️ Cloud-init source: {}", path.display()),
        CloudInitEvent::SourceInline => info!("☁️ Cloud-init source: inline content"),
        CloudInitEvent::DryRunTarget(path) => {
            info!("🧪 Dry run: cloud-init target would be created at: {}", path.display());
        }
        CloudInitEvent::TargetWritten(path) => info!("☁️ Cloud-init target: {}", path.display()),
        CloudInitEvent::DebugCopyWritten(path) => info!("☁️ Cloud-init debug copy: {}", path.display()),
        CloudInitEvent::DebugCopySkipped(reason) => {
            info!("☁️ Cloud-init debug copy skipped ({reason})");
        }
    }
}
