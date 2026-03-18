//! Entry point for the cloud-init provisioning flow.
//!
//! `prepare_cloud_init` is the single function called by `WslManager` to drive
//! the full load → render → store sequence. The submodules handle each step
//! independently; this module wires them together and tracks provisioning events.

mod load;
mod render;
mod store;

use crate::config::{CloudInitSource, Profile};
use crate::wsl::helpers::path::resolve_userprofile_dir;
use crate::wsl::model::Event;
use std::path::PathBuf;

pub use store::DebugCopyOutcome;

pub fn cloud_init_target_file(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(profile: &Profile, dry_run: bool, debug: bool) -> anyhow::Result<Vec<Event>> {
    let mut events = Vec::new();
    let Some(source) = &profile.cloud_init else {
        events.push(Event::CloudInitSkipped);
        return Ok(events);
    };

    let content = match source {
        CloudInitSource::File { path } => {
            events.push(Event::CloudInitSourceResolved(path.clone()));
            load::load_cloud_init_source(source)?
        }
        CloudInitSource::Inline { content } => {
            events.push(Event::CloudInitInlineLoaded);
            content.clone()
        }
    };
    let rendered = render::render(&content, profile)?;

    let target_file = cloud_init_target_file(&profile.hostname)?;
    if dry_run {
        events.push(Event::CloudInitDryRunDeployed(target_file));
        return Ok(events);
    }

    store::store(&target_file, &rendered)?;
    events.push(Event::CloudInitDeployed(target_file));
    if debug {
        match store::copy_debug_to_current_dir(&profile.hostname, &rendered) {
            DebugCopyOutcome::Written(path) => events.push(Event::CloudInitDebugCopied(path)),
            DebugCopyOutcome::Skipped(reason) => events.push(Event::CloudInitDebugSkipped(reason)),
        }
    }
    Ok(events)
}
