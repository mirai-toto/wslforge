use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateOutcome {
    Created,
    AlreadyExists,
    Skipped,
}

impl CreateOutcome {
    pub fn icon(&self) -> ReportIcon {
        match self {
            Self::Created => ReportIcon::Success,
            Self::AlreadyExists => ReportIcon::Info,
            Self::Skipped => ReportIcon::Info,
        }
    }

    pub fn message(&self, hostname: &str) -> String {
        match self {
            Self::Created => format!("WSL instance '{}' created successfully.", hostname),
            Self::AlreadyExists => format!("WSL instance '{}' already exists.", hostname),
            Self::Skipped => format!("WSL instance '{}' was skipped.", hostname),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloudInitEvent {
    NotConfigured,
    SourceFile(PathBuf),
    SourceInline,
    DryRunTarget(PathBuf),
    TargetWritten(PathBuf),
    DebugCopyWritten(PathBuf),
    DebugCopySkipped(String),
}

impl CloudInitEvent {
    pub fn icon(&self) -> ReportIcon {
        match self {
            Self::DryRunTarget(_) => ReportIcon::DryRun,
            Self::NotConfigured
            | Self::SourceFile(_)
            | Self::SourceInline
            | Self::TargetWritten(_)
            | Self::DebugCopyWritten(_)
            | Self::DebugCopySkipped(_) => ReportIcon::CloudInit,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::NotConfigured => "Cloud-init: not configured".to_string(),
            Self::SourceFile(path) => format!("Cloud-init source: {}", path.display()),
            Self::SourceInline => "Cloud-init source: inline content".to_string(),
            Self::DryRunTarget(path) => {
                format!("Dry run: cloud-init target would be created at: {}", path.display())
            }
            Self::TargetWritten(path) => format!("Cloud-init target: {}", path.display()),
            Self::DebugCopyWritten(path) => format!("Cloud-init debug copy: {}", path.display()),
            Self::DebugCopySkipped(reason) => format!("Cloud-init debug copy skipped ({reason})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateEvent {
    InstanceCheckStarted,
    InstanceExists,
    InstanceMissing,
    OverrideRequested,
    OverrideExistingInstance,
    DeleteSkippedMissing,
    DeleteDryRun,
    DeleteStarted,
    DeleteCompleted,
    CreateDryRun,
    CreateStarted,
    CloudInit(CloudInitEvent),
}

impl CreateEvent {
    pub fn icon(&self) -> ReportIcon {
        match self {
            Self::InstanceCheckStarted => ReportIcon::Search,
            Self::InstanceExists => ReportIcon::Success,
            Self::InstanceMissing => ReportIcon::Info,
            Self::OverrideRequested => ReportIcon::Info,
            Self::OverrideExistingInstance => ReportIcon::Warning,
            Self::DeleteSkippedMissing => ReportIcon::Info,
            Self::DeleteDryRun => ReportIcon::DryRun,
            Self::DeleteStarted => ReportIcon::Delete,
            Self::DeleteCompleted => ReportIcon::Success,
            Self::CreateDryRun => ReportIcon::DryRun,
            Self::CreateStarted => ReportIcon::Create,
            Self::CloudInit(event) => event.icon(),
        }
    }

    pub fn message(&self, hostname: &str) -> Option<String> {
        match self {
            Self::InstanceCheckStarted => Some(format!("Checking if WSL instance '{}' exists...", hostname)),
            Self::InstanceExists => Some(format!("WSL instance '{}' exists.", hostname)),
            Self::InstanceMissing => Some(format!("WSL instance '{}' does not exist.", hostname)),
            Self::OverrideRequested => None,
            Self::OverrideExistingInstance => Some(format!(
                "WSL instance '{}' already exists and will be overridden.",
                hostname
            )),
            Self::DeleteSkippedMissing => Some(format!("WSL instance '{}' does not exist. Skipping delete.", hostname)),
            Self::DeleteDryRun => Some(format!("Dry run: WSL instance '{}' would be deleted", hostname)),
            Self::DeleteStarted => Some(format!("Deleting existing WSL instance '{}'", hostname)),
            Self::DeleteCompleted => Some(format!("WSL instance '{}' deleted successfully.", hostname)),
            Self::CreateDryRun => Some("Dry run: WSL instance would be created".to_string()),
            Self::CreateStarted => Some("Creating WSL instance".to_string()),
            Self::CloudInit(event) => Some(event.message()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateReport {
    pub outcome: CreateOutcome,
    pub events: Vec<CreateEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvironmentEvent {
    WslInstalled,
    WslUpdateDryRun,
    WslUpdateCompleted,
    WindowsFeatureEnabled(String),
}

impl EnvironmentEvent {
    pub fn icon(&self) -> ReportIcon {
        match self {
            Self::WslUpdateDryRun => ReportIcon::DryRun,
            Self::WslInstalled | Self::WslUpdateCompleted | Self::WindowsFeatureEnabled(_) => ReportIcon::Success,
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::WslInstalled => "WSL is installed".to_string(),
            Self::WslUpdateDryRun => "Dry run: WSL update would be performed".to_string(),
            Self::WslUpdateCompleted => "WSL update completed".to_string(),
            Self::WindowsFeatureEnabled(name) => format!("{name} is enabled"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentReport {
    pub events: Vec<EnvironmentEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportIcon {
    Search,
    Success,
    Info,
    Warning,
    DryRun,
    Delete,
    Create,
    CloudInit,
}

impl fmt::Display for ReportIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self {
            Self::Search => "🔍",
            Self::Success => "✅",
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::DryRun => "🧪",
            Self::Delete => "🧹",
            Self::Create => "🚀",
            Self::CloudInit => "☁️",
        };
        write!(f, "{icon}")
    }
}
