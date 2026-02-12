use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExecutionOptions {
    pub dry_run: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateOutcome {
    Created,
    AlreadyExists,
    Skipped,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentReport {
    pub events: Vec<EnvironmentEvent>,
}
