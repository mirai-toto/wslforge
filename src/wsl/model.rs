use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RunOptions {
    pub dry_run: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Created,
    AlreadyExists,
    Skipped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisionEvent {
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
    CloudInitNotConfigured,
    CloudInitSourceFile(PathBuf),
    CloudInitSourceInline,
    CloudInitDryRunTarget(PathBuf),
    CloudInitTargetWritten(PathBuf),
    CloudInitDebugCopyWritten(PathBuf),
    CloudInitDebugCopySkipped(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileResult {
    pub outcome: Outcome,
    pub events: Vec<ProvisionEvent>,
}
