use crate::config::CloudInitSource;
use crate::wsl::helpers::path::expand_env_vars;
use log::info;
use std::path::PathBuf;

pub(super) fn load_cloud_init_source(source: &CloudInitSource) -> anyhow::Result<String> {
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
