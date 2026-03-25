use serde::Deserialize;
use std::fmt;

use super::source::SourcePath;

fn default_distro() -> String {
    "Ubuntu".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ImageSource {
    Distro {
        #[serde(default = "default_distro")]
        name: String,
    },
    File {
        path: SourcePath,
    },
}

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageSource::Distro { name } => write!(f, "distro: {name}"),
            ImageSource::File { path } => write!(f, "file: {path}"),
        }
    }
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Distro { name: default_distro() }
    }
}
