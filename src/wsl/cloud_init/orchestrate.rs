use crate::config::{CloudInitInput, Profile};
use crate::wsl::helpers::userprofile::resolve_userprofile_dir;
use crate::wsl::model::CloudInitEvent;
use std::path::PathBuf;

use super::{
    copy_debug_to_current_dir,
    load::{LoadedCloudInitSource, load_cloud_init_source},
    render, store, DebugCopyOutcome,
};

pub fn cloud_init_target_file(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(
    profile: &Profile,
    dry_run: bool,
    debug: bool,
) -> anyhow::Result<Vec<CloudInitEvent>> {
    let mut events = Vec::new();
    let Some(source) = &profile.cloud_init else {
        events.push(CloudInitEvent::NotConfigured);
        return Ok(events);
    };

    let LoadedCloudInitSource { content, source } = load_cloud_init_source(source)?;
    match source {
        CloudInitInput::File { path } => events.push(CloudInitEvent::SourceFile(path)),
        CloudInitInput::Inline { .. } => events.push(CloudInitEvent::SourceInline),
    }
    let rendered = render(&content, profile)?;

    let target_file = cloud_init_target_file(&profile.hostname)?;
    if dry_run {
        events.push(CloudInitEvent::DryRunTarget(target_file));
        return Ok(events);
    }

    store(&target_file, &rendered)?;
    events.push(CloudInitEvent::TargetWritten(target_file));
    if debug {
        match copy_debug_to_current_dir(&profile.hostname, &rendered) {
            DebugCopyOutcome::Written(path) => events.push(CloudInitEvent::DebugCopyWritten(path)),
            DebugCopyOutcome::Skipped(reason) => {
                events.push(CloudInitEvent::DebugCopySkipped(reason))
            }
        }
    }
    Ok(events)
}
