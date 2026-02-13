use crate::config::{CloudInitInput, Profile};
use crate::wsl::helpers::userprofile::resolve_userprofile_dir;
use crate::wsl::model::ProfileEvent;
use std::path::PathBuf;

use super::{
    copy_debug_to_current_dir,
    load::load_cloud_init_source,
    render, store, DebugCopyOutcome,
};

pub fn cloud_init_target_file(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(profile: &Profile, dry_run: bool, debug: bool) -> anyhow::Result<Vec<ProfileEvent>> {
    let mut events = Vec::new();
    let Some(source) = &profile.cloud_init else {
        events.push(ProfileEvent::CloudInitNotConfigured);
        return Ok(events);
    };

    let content = match source {
        CloudInitInput::File { path } => {
            events.push(ProfileEvent::CloudInitSourceFile(path.clone()));
            load_cloud_init_source(source)?
        }
        CloudInitInput::Inline { content } => {
            events.push(ProfileEvent::CloudInitSourceInline);
            content.clone()
        }
    };
    let rendered = render(&content, profile)?;

    let target_file = cloud_init_target_file(&profile.hostname)?;
    if dry_run {
        events.push(ProfileEvent::CloudInitDryRunTarget(target_file));
        return Ok(events);
    }

    store(&target_file, &rendered)?;
    events.push(ProfileEvent::CloudInitTargetWritten(target_file));
    if debug {
        match copy_debug_to_current_dir(&profile.hostname, &rendered) {
            DebugCopyOutcome::Written(path) => events.push(ProfileEvent::CloudInitDebugCopyWritten(path)),
            DebugCopyOutcome::Skipped(reason) => events.push(ProfileEvent::CloudInitDebugCopySkipped(reason)),
        }
    }
    Ok(events)
}
