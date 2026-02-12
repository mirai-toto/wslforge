use crate::wsl::{CloudInitEvent, CreateEvent, CreateOutcome, CreateReport};
use log::info;

struct LogEntry {
    icon: &'static str,
    message: Option<String>,
}

pub fn log_create_report(report: &CreateReport, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    log_create_outcome(report.outcome, hostname);
}

fn log_create_event(event: &CreateEvent, hostname: &str) {
    let formatted = format_create_event(event, hostname);
    if let Some(message) = formatted.message {
        info!("{} {}", formatted.icon, message);
    }
}

fn log_create_outcome(outcome: CreateOutcome, hostname: &str) {
    info!(
        "{} {}",
        create_outcome_icon(outcome),
        create_outcome_message(outcome, hostname)
    );
}

fn create_outcome_icon(outcome: CreateOutcome) -> &'static str {
    match outcome {
        CreateOutcome::Created => "✅",
        CreateOutcome::AlreadyExists | CreateOutcome::Skipped => "ℹ️",
    }
}

fn create_outcome_message(outcome: CreateOutcome, hostname: &str) -> String {
    match outcome {
        CreateOutcome::Created => format!("WSL instance '{}' created successfully.", hostname),
        CreateOutcome::AlreadyExists => format!("WSL instance '{}' already exists.", hostname),
        CreateOutcome::Skipped => format!("WSL instance '{}' was skipped.", hostname),
    }
}

fn format_create_event(event: &CreateEvent, hostname: &str) -> LogEntry {
    match event {
        CreateEvent::InstanceCheckStarted | CreateEvent::InstanceExists | CreateEvent::InstanceMissing => {
            format_instance_event(event, hostname)
        }
        CreateEvent::OverrideRequested | CreateEvent::OverrideExistingInstance => {
            format_override_event(event, hostname)
        }
        CreateEvent::DeleteSkippedMissing
        | CreateEvent::DeleteDryRun
        | CreateEvent::DeleteStarted
        | CreateEvent::DeleteCompleted => format_delete_event(event, hostname),
        CreateEvent::CreateDryRun | CreateEvent::CreateStarted => format_create_step_event(event),
        CreateEvent::CloudInit(event) => format_cloud_init_event(event),
    }
}

fn format_instance_event(event: &CreateEvent, hostname: &str) -> LogEntry {
    match event {
        CreateEvent::InstanceCheckStarted => LogEntry {
            icon: "🔍",
            message: Some(format!("Checking if WSL instance '{}' exists...", hostname)),
        },
        CreateEvent::InstanceExists => LogEntry {
            icon: "✅",
            message: Some(format!("WSL instance '{}' exists.", hostname)),
        },
        CreateEvent::InstanceMissing => LogEntry {
            icon: "ℹ️",
            message: Some(format!("WSL instance '{}' does not exist.", hostname)),
        },
        _ => unreachable!("format_instance_event called with non-instance event"),
    }
}

fn format_override_event(event: &CreateEvent, hostname: &str) -> LogEntry {
    match event {
        CreateEvent::OverrideRequested => LogEntry {
            icon: "ℹ️",
            message: None,
        },
        CreateEvent::OverrideExistingInstance => LogEntry {
            icon: "⚠️",
            message: Some(format!(
                "WSL instance '{}' already exists and will be overridden.",
                hostname
            )),
        },
        _ => unreachable!("format_override_event called with non-override event"),
    }
}

fn format_delete_event(event: &CreateEvent, hostname: &str) -> LogEntry {
    match event {
        CreateEvent::DeleteSkippedMissing => LogEntry {
            icon: "ℹ️",
            message: Some(format!("WSL instance '{}' does not exist. Skipping delete.", hostname)),
        },
        CreateEvent::DeleteDryRun => LogEntry {
            icon: "🧪",
            message: Some(format!("Dry run: WSL instance '{}' would be deleted", hostname)),
        },
        CreateEvent::DeleteStarted => LogEntry {
            icon: "🧹",
            message: Some(format!("Deleting existing WSL instance '{}'", hostname)),
        },
        CreateEvent::DeleteCompleted => LogEntry {
            icon: "✅",
            message: Some(format!("WSL instance '{}' deleted successfully.", hostname)),
        },
        _ => unreachable!("format_delete_event called with non-delete event"),
    }
}

fn format_create_step_event(event: &CreateEvent) -> LogEntry {
    match event {
        CreateEvent::CreateDryRun => LogEntry {
            icon: "🧪",
            message: Some("Dry run: WSL instance would be created".to_string()),
        },
        CreateEvent::CreateStarted => LogEntry {
            icon: "🚀",
            message: Some("Creating WSL instance".to_string()),
        },
        _ => unreachable!("format_create_step_event called with non-create event"),
    }
}

fn format_cloud_init_event(event: &CloudInitEvent) -> LogEntry {
    match event {
        CloudInitEvent::NotConfigured => LogEntry {
            icon: "☁️",
            message: Some("Cloud-init: not configured".to_string()),
        },
        CloudInitEvent::SourceFile(path) => LogEntry {
            icon: "☁️",
            message: Some(format!("Cloud-init source: {}", path.display())),
        },
        CloudInitEvent::SourceInline => LogEntry {
            icon: "☁️",
            message: Some("Cloud-init source: inline content".to_string()),
        },
        CloudInitEvent::DryRunTarget(path) => LogEntry {
            icon: "🧪",
            message: Some(format!(
                "Dry run: cloud-init target would be created at: {}",
                path.display()
            )),
        },
        CloudInitEvent::TargetWritten(path) => LogEntry {
            icon: "☁️",
            message: Some(format!("Cloud-init target: {}", path.display())),
        },
        CloudInitEvent::DebugCopyWritten(path) => LogEntry {
            icon: "☁️",
            message: Some(format!("Cloud-init debug copy: {}", path.display())),
        },
        CloudInitEvent::DebugCopySkipped(reason) => LogEntry {
            icon: "☁️",
            message: Some(format!("Cloud-init debug copy skipped ({reason})")),
        },
    }
}
