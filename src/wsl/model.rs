use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunOptions {
    pub dry_run: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Created,
    Recreated,
    AlreadyExists,
    Skipped,
    Failed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    InstanceCheckStarted,
    InstanceFound,
    InstanceNotFound,
    OverrideEnabled,
    OverrideTriggered,
    DeleteSkipped,
    DeleteDryRun,
    DeleteStarted,
    DeleteCompleted,
    CreateDryRun,
    CreateStarted,
    CloudInitSkipped,
    CloudInitSourceResolved(PathBuf),
    CloudInitInlineLoaded,
    CloudInitDryRunDeployed(PathBuf),
    CloudInitDeployed(PathBuf),
    CloudInitDebugCopied(PathBuf),
    CloudInitDebugSkipped(String),
    FileTransferStarted(PathBuf),
    FileTransferCompleted(String),
    ImageDownloadStarted,
    ImageDownloadCompleted,
    ScriptStarted(String),
    ScriptCompleted(String),
}

impl Event {
    pub fn describe(&self, hostname: &str) -> String {
        match self {
            Event::InstanceCheckStarted => format!("Checking if WSL instance '{}' exists...", hostname),
            Event::InstanceFound => format!("WSL instance '{}' exists.", hostname),
            Event::InstanceNotFound => format!("WSL instance '{}' does not exist.", hostname),
            Event::OverrideEnabled => format!("Override requested for WSL instance '{}'.", hostname),
            Event::OverrideTriggered => {
                format!("WSL instance '{}' already exists and will be overridden.", hostname)
            }
            Event::DeleteSkipped => format!("WSL instance '{}' does not exist. Skipping delete.", hostname),
            Event::DeleteDryRun => format!("Dry run: WSL instance '{}' would be deleted", hostname),
            Event::DeleteStarted => format!("Deleting existing WSL instance '{}'", hostname),
            Event::DeleteCompleted => format!("WSL instance '{}' deleted successfully.", hostname),
            Event::CreateDryRun => format!("Dry run: WSL instance '{}' would be created", hostname),
            Event::CreateStarted => format!("Creating WSL instance '{}'", hostname),
            Event::CloudInitSkipped => "Cloud-init: not configured".to_string(),
            Event::CloudInitSourceResolved(path) => format!("Cloud-init source: {}", path.display()),
            Event::CloudInitInlineLoaded => "Cloud-init source: inline content".to_string(),
            Event::CloudInitDryRunDeployed(path) => {
                format!("Dry run: cloud-init target would be created at: {}", path.display())
            }
            Event::CloudInitDeployed(path) => format!("Cloud-init target: {}", path.display()),
            Event::CloudInitDebugCopied(path) => format!("Cloud-init debug copy: {}", path.display()),
            Event::CloudInitDebugSkipped(reason) => format!("Cloud-init debug copy skipped ({reason})"),
            Event::FileTransferStarted(src) => format!("Transferring file: {}", src.display()),
            Event::FileTransferCompleted(dest) => format!("File transferred to: {dest}"),
            Event::ImageDownloadStarted => "Downloading image...".to_string(),
            Event::ImageDownloadCompleted => "Image downloaded.".to_string(),
            Event::ScriptStarted(cmd) => format!("Running script: {cmd}"),
            Event::ScriptCompleted(cmd) => format!("Script completed: {cmd}"),
        }
    }
}

impl Status {
    pub fn describe(&self, hostname: &str) -> String {
        match self {
            Status::Created => format!("WSL instance '{}' created successfully.", hostname),
            Status::Recreated => format!("WSL instance '{}' recreated successfully.", hostname),
            Status::AlreadyExists => format!("WSL instance '{}' already exists.", hostname),
            Status::Skipped => format!("WSL instance '{}' was skipped.", hostname),
            Status::Failed(e) => format!("WSL instance '{}' failed: {e}", hostname),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceResult {
    pub hostname: String,
    pub outcome: Status,
    pub events: Vec<Event>,
}

impl InstanceResult {
    pub fn log(&self) {
        for event in &self.events {
            log::debug!(target: "wslforge::events", "{}", event.describe(&self.hostname));
        }
        log::debug!(target: "wslforge::events", "{}", self.outcome.describe(&self.hostname));
    }
}
