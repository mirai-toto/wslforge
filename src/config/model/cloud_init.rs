use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;

fn default_cloud_init_path() -> PathBuf {
    "cloud-init.yaml".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CloudInitSource {
    File {
        #[serde(default = "default_cloud_init_path")]
        path: PathBuf,
    },
    Inline {
        content: String,
    },
}

impl fmt::Display for CloudInitSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudInitSource::File { path } => write!(f, "file: {}", path.display()),
            CloudInitSource::Inline { .. } => write!(f, "inline"),
        }
    }
}
