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
    CloudInitDefaultGenerated,
    CloudInitSourceResolved(PathBuf),
    CloudInitInlineLoaded,
    CloudInitDryRunDeployed(PathBuf),
    CloudInitDeployed(PathBuf),
    CloudInitDebugCopied(PathBuf),
    CloudInitDebugSkipped(String),
    FileTransferStarted(PathBuf),
    FileTransferCompleted(String),
    DirectoryTransferStarted(PathBuf),
    DirectoryTransferCompleted(String),
    ImageDownloadStarted,
    ImageDownloadCompleted,
    ProvisioningWaiting,
    ProvisioningCompleted,
    ScriptStarted(String),
    ScriptCompleted(String),
}

impl Event {
    pub fn describe(&self, name: &str) -> String {
        match self {
            Event::InstanceCheckStarted => format!("Checking if WSL instance '{}' exists...", name),
            Event::InstanceFound => format!("WSL instance '{}' exists.", name),
            Event::InstanceNotFound => format!("WSL instance '{}' does not exist.", name),
            Event::OverrideEnabled => format!("Override requested for WSL instance '{}'.", name),
            Event::OverrideTriggered => {
                format!("WSL instance '{}' already exists and will be overridden.", name)
            }
            Event::DeleteSkipped => format!("WSL instance '{}' does not exist. Skipping delete.", name),
            Event::DeleteDryRun => format!("Dry run: WSL instance '{}' would be deleted", name),
            Event::DeleteStarted => format!("Deleting existing WSL instance '{}'", name),
            Event::DeleteCompleted => format!("WSL instance '{}' deleted successfully.", name),
            Event::CreateDryRun => format!("Dry run: WSL instance '{}' would be created", name),
            Event::CreateStarted => format!("Creating WSL instance '{}'", name),
            Event::CloudInitSkipped => "Cloud-init: not configured".to_string(),
            Event::CloudInitDefaultGenerated => "Cloud-init: no config provided, using generated default".to_string(),
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
            Event::DirectoryTransferStarted(src) => format!("Transferring directory: {}", src.display()),
            Event::DirectoryTransferCompleted(dest) => format!("Directory transferred to: {dest}"),
            Event::ImageDownloadStarted => "Downloading image...".to_string(),
            Event::ImageDownloadCompleted => "Image downloaded.".to_string(),
            Event::ProvisioningWaiting => format!("Waiting for cloud-init to complete on '{name}'..."),
            Event::ProvisioningCompleted => format!("Cloud-init provisioning completed on '{name}'."),
            Event::ScriptStarted(cmd) => format!("Running script: {cmd}"),
            Event::ScriptCompleted(cmd) => format!("Script completed: {cmd}"),
        }
    }
}

impl Status {
    pub fn describe(&self, name: &str) -> String {
        match self {
            Status::Created => format!("WSL instance '{}' created successfully.", name),
            Status::Recreated => format!("WSL instance '{}' recreated successfully.", name),
            Status::AlreadyExists => format!("WSL instance '{}' already exists.", name),
            Status::Skipped => format!("WSL instance '{}' was skipped.", name),
            Status::Failed(e) => format!("WSL instance '{}' failed: {e}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceResult {
    pub name: String,
    pub outcome: Status,
    pub events: Vec<Event>,
}

impl InstanceResult {
    pub fn log(&self) {
        for event in &self.events {
            log::debug!(target: "wslforge::events", "{}", event.describe(&self.name));
        }
        log::debug!(target: "wslforge::events", "{}", self.outcome.describe(&self.name));
    }
}
