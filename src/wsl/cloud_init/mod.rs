use super::helpers::path::expand_env_vars;
use crate::config::{CloudInitSource, Profile};
use log::{debug, info};
use std::path::PathBuf;

mod render;
mod store;

pub use render::render;
pub use store::store;

pub fn prepare_cloud_init(profile: &Profile, dry_run: bool, debug: bool) -> anyhow::Result<()> {
    let Some(source) = &profile.cloud_init else {
        info!("☁️ Cloud-init: not configured");
        return Ok(());
    };

    let raw = load_cloud_init_source(source)?;
    let rendered = render(&raw, profile)?;
    debug!("☁️ Cloud-init rendered:\n{}", rendered);
    let target_file = store(&profile.hostname, &rendered, dry_run, debug)?;
    info!("☁️ Cloud-init target: {}", target_file.display());
    Ok(())
}

fn load_cloud_init_source(source: &CloudInitSource) -> anyhow::Result<String> {
    match source {
        CloudInitSource::File { path } => {
            let expanded = expand_env_vars(&path.to_string_lossy())?;
            let expanded_path = PathBuf::from(expanded);
            if !expanded_path.exists() {
                anyhow::bail!("cloud-init user-data file not found: {}", expanded_path.display());
            }
            info!("☁️ Cloud-init source: {}", expanded_path.display());
            std::fs::read_to_string(expanded_path).map_err(Into::into)
        }
        CloudInitSource::Inline { content } => {
            info!("☁️ Cloud-init source: inline content");
            Ok(content.to_string())
        }
    }
}
