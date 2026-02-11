use crate::config::{ImageSource, Profile};
use crate::wsl::helpers::path;
use crate::wsl::{CloudInitEvent, CreateEvent, CreateOutcome, CreateReport, EnvironmentEvent, EnvironmentReport};
use log::info;

pub fn log_create_report(report: &CreateReport, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    log_create_outcome(report.outcome, hostname);
}

pub fn log_environment_report(report: &EnvironmentReport) {
    for event in &report.events {
        log_environment_event(event);
    }
}

pub fn log_config_summary(profile_name: &str, profile: &Profile) {
    info!("🧩 Profile: {}", profile_name);
    info!("♻️ Override: {}", profile.override_instance);
    info!("🏷️ Hostname: {}", profile.hostname);
    info!("👤 User: {}", profile.username);
    info!("📦 Install dir: {}", expand_install_dir(profile));
    match &profile.cloud_init {
        Some(source) => info!("☁️ Cloud-init: {}", source),
        None => info!("☁️ Cloud-init: not configured"),
    }

    match &profile.image {
        ImageSource::Distro { name } => {
            info!("🐧 Using WSL distro '{}'", name);
        }
        ImageSource::File { path } => {
            info!("🗂️  Using image file {:?}", path);
        }
    }

    if let Some(proxy) = &profile.http_proxy {
        info!("🌐 HTTP proxy: {}", proxy);
    }
    if let Some(proxy) = &profile.https_proxy {
        info!("🔐 HTTPS proxy: {}", proxy);
    }
}

fn expand_install_dir(profile: &Profile) -> String {
    match path::expand_path(&profile.install_dir) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => profile.install_dir.to_string_lossy().into_owned(),
    }
}

fn log_environment_event(event: &EnvironmentEvent) {
    info!("{} {}", environment_event_icon(event), environment_event_message(event));
}

fn log_create_event(event: &CreateEvent, hostname: &str) {
    if let Some(message) = create_event_message(event, hostname) {
        info!("{} {}", create_event_icon(event), message);
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

fn create_event_icon(event: &CreateEvent) -> &'static str {
    match event {
        CreateEvent::InstanceCheckStarted => "🔍",
        CreateEvent::InstanceExists => "✅",
        CreateEvent::InstanceMissing | CreateEvent::OverrideRequested | CreateEvent::DeleteSkippedMissing => "ℹ️",
        CreateEvent::OverrideExistingInstance => "⚠️",
        CreateEvent::DeleteDryRun | CreateEvent::CreateDryRun => "🧪",
        CreateEvent::DeleteStarted => "🧹",
        CreateEvent::DeleteCompleted => "✅",
        CreateEvent::CreateStarted => "🚀",
        CreateEvent::CloudInit(event) => cloud_init_event_icon(event),
    }
}

fn create_event_message(event: &CreateEvent, hostname: &str) -> Option<String> {
    match event {
        CreateEvent::InstanceCheckStarted => Some(format!("Checking if WSL instance '{}' exists...", hostname)),
        CreateEvent::InstanceExists => Some(format!("WSL instance '{}' exists.", hostname)),
        CreateEvent::InstanceMissing => Some(format!("WSL instance '{}' does not exist.", hostname)),
        CreateEvent::OverrideRequested => None,
        CreateEvent::OverrideExistingInstance => Some(format!(
            "WSL instance '{}' already exists and will be overridden.",
            hostname
        )),
        CreateEvent::DeleteSkippedMissing => {
            Some(format!("WSL instance '{}' does not exist. Skipping delete.", hostname))
        }
        CreateEvent::DeleteDryRun => Some(format!("Dry run: WSL instance '{}' would be deleted", hostname)),
        CreateEvent::DeleteStarted => Some(format!("Deleting existing WSL instance '{}'", hostname)),
        CreateEvent::DeleteCompleted => Some(format!("WSL instance '{}' deleted successfully.", hostname)),
        CreateEvent::CreateDryRun => Some("Dry run: WSL instance would be created".to_string()),
        CreateEvent::CreateStarted => Some("Creating WSL instance".to_string()),
        CreateEvent::CloudInit(event) => Some(cloud_init_event_message(event)),
    }
}

fn cloud_init_event_icon(event: &CloudInitEvent) -> &'static str {
    match event {
        CloudInitEvent::DryRunTarget(_) => "🧪",
        CloudInitEvent::NotConfigured
        | CloudInitEvent::SourceFile(_)
        | CloudInitEvent::SourceInline
        | CloudInitEvent::TargetWritten(_)
        | CloudInitEvent::DebugCopyWritten(_)
        | CloudInitEvent::DebugCopySkipped(_) => "☁️",
    }
}

fn cloud_init_event_message(event: &CloudInitEvent) -> String {
    match event {
        CloudInitEvent::NotConfigured => "Cloud-init: not configured".to_string(),
        CloudInitEvent::SourceFile(path) => format!("Cloud-init source: {}", path.display()),
        CloudInitEvent::SourceInline => "Cloud-init source: inline content".to_string(),
        CloudInitEvent::DryRunTarget(path) => {
            format!("Dry run: cloud-init target would be created at: {}", path.display())
        }
        CloudInitEvent::TargetWritten(path) => format!("Cloud-init target: {}", path.display()),
        CloudInitEvent::DebugCopyWritten(path) => format!("Cloud-init debug copy: {}", path.display()),
        CloudInitEvent::DebugCopySkipped(reason) => format!("Cloud-init debug copy skipped ({reason})"),
    }
}

fn environment_event_icon(event: &EnvironmentEvent) -> &'static str {
    match event {
        EnvironmentEvent::WslUpdateDryRun => "🧪",
        EnvironmentEvent::WslInstalled
        | EnvironmentEvent::WslUpdateCompleted
        | EnvironmentEvent::WindowsFeatureEnabled(_) => "✅",
    }
}

fn environment_event_message(event: &EnvironmentEvent) -> String {
    match event {
        EnvironmentEvent::WslInstalled => "WSL is installed".to_string(),
        EnvironmentEvent::WslUpdateDryRun => "Dry run: WSL update would be performed".to_string(),
        EnvironmentEvent::WslUpdateCompleted => "WSL update completed".to_string(),
        EnvironmentEvent::WindowsFeatureEnabled(name) => format!("{name} is enabled"),
    }
}
