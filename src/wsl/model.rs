use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunOptions {
    pub dry_run: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Created,
    AlreadyExists,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    InstanceCheckStarted,
    InstanceFound,
    InstanceNotFound,
    OverrideRequested,
    OverrideStarted,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileResult {
    pub outcome: Status,
    pub events: Vec<Event>,
}
