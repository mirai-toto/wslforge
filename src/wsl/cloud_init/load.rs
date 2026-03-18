use crate::config::CloudInitSource;
use crate::wsl::helpers::expand_env_vars;
use std::path::PathBuf;

pub(super) fn load_cloud_init_source(source: &CloudInitSource) -> anyhow::Result<String> {
    match source {
        CloudInitSource::File { path } => {
            let expanded = expand_env_vars(&path.to_string_lossy())?;
            let expanded_path = PathBuf::from(expanded);
            if !expanded_path.exists() {
                anyhow::bail!("cloud-init user-data file not found: {}", expanded_path.display());
            }
            let content = std::fs::read_to_string(&expanded_path)?;
            Ok(content)
        }
        CloudInitSource::Inline { content } => Ok(content.to_string()),
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/load_tests.rs"]
mod load_tests;
