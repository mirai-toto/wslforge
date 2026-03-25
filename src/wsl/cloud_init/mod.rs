//! Entry point for the cloud-init provisioning flow.
//!
//! `prepare_cloud_init` is the single function called by `WslManager` to drive
//! the full load → render → store sequence. The submodules handle each step
//! independently; this module wires them together and tracks provisioning events.

mod load;
mod render;
mod store;

use crate::config::{CloudInitSource, Instance};
use crate::wsl::helpers::resolve_userprofile_dir;
use crate::wsl::model::Event;
use std::path::PathBuf;

pub use store::DebugCopyOutcome;

pub fn user_data_path(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(instance: &Instance, dry_run: bool, debug: bool) -> anyhow::Result<Vec<Event>> {
    let Some(source) = &instance.cloud_init else {
        return Ok(vec![Event::CloudInitSkipped]);
    };
    let mut events: Vec<Event> = Vec::new();

    let content: String = match source {
        CloudInitSource::File { path } => {
            events.push(Event::CloudInitSourceResolved(path.clone()));
            load::load_cloud_init_source(path)?
        }
        CloudInitSource::Inline { content } => {
            events.push(Event::CloudInitInlineLoaded);
            content.clone()
        }
    };
    let rendered: String = render::render(&content, instance)?;

    let target_file: PathBuf = user_data_path(&instance.hostname)?;
    if dry_run {
        events.push(Event::CloudInitDryRunDeployed(target_file));
        return Ok(events);
    }

    store::store(&target_file, &rendered)?;
    events.push(Event::CloudInitDeployed(target_file));
    if debug {
        match store::copy_debug_to_current_dir(&instance.hostname, &rendered) {
            DebugCopyOutcome::Written(path) => events.push(Event::CloudInitDebugCopied(path)),
            DebugCopyOutcome::Skipped(reason) => events.push(Event::CloudInitDebugSkipped(reason)),
        }
    }
    Ok(events)
}
