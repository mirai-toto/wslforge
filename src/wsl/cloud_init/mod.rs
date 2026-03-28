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

pub const DEFAULT_CLOUD_INIT_TEMPLATE: &str = r#"#cloud-config
users:
  - name: {{ username }}
    groups: [sudo]
    shell: /bin/bash
    sudo: ALL=(ALL) NOPASSWD:ALL
{%- if password_hash %}
    passwd: {{ password_hash }}
    lock_passwd: false
{%- endif %}
{% if proxy %}
write_files:
  - path: /etc/environment
    append: true
    content: |
{% if proxy.http %}
      http_proxy={{ proxy.http }}
      HTTP_PROXY={{ proxy.http }}
{% endif %}
{% if proxy.https %}
      https_proxy={{ proxy.https }}
      HTTPS_PROXY={{ proxy.https }}
{% endif %}
{% if proxy.no_proxy %}
      no_proxy={{ proxy.no_proxy }}
      NO_PROXY={{ proxy.no_proxy }}
{% endif %}
{% endif %}"#;

pub fn user_data_path(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(instance: &Instance, dry_run: bool, debug: bool) -> anyhow::Result<Vec<Event>> {
    let mut events: Vec<Event> = Vec::new();

    let default_source;
    let source = match &instance.cloud_init {
        Some(s) => s,
        None if !instance.default_cloud_init => return Ok(events),
        None => {
            events.push(Event::CloudInitDefaultGenerated);
            default_source = CloudInitSource::Inline {
                content: DEFAULT_CLOUD_INIT_TEMPLATE.into(),
            };
            &default_source
        }
    };

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
