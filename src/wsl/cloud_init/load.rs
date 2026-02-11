use crate::config::CloudInitInput;
use crate::wsl::helpers::path::expand_env_vars;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(super) struct LoadedCloudInitSource {
    pub content: String,
    pub source: CloudInitInput,
}

pub(super) fn load_cloud_init_source(source: &CloudInitInput) -> anyhow::Result<LoadedCloudInitSource> {
    match source {
        CloudInitInput::File { path } => {
            let expanded = expand_env_vars(&path.to_string_lossy())?;
            let expanded_path = PathBuf::from(expanded);
            if !expanded_path.exists() {
                anyhow::bail!("cloud-init user-data file not found: {}", expanded_path.display());
            }
            let content = std::fs::read_to_string(&expanded_path)?;
            Ok(LoadedCloudInitSource {
                content,
                source: CloudInitInput::File { path: expanded_path },
            })
        }
        CloudInitInput::Inline { content } => Ok(LoadedCloudInitSource {
            content: content.to_string(),
            source: CloudInitInput::Inline {
                content: content.clone(),
            },
        }),
    }
}
