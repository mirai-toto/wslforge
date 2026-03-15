use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use url::Url;

fn default_hostname() -> String {
    "UbuntuWSL".into()
}

fn default_username() -> String {
    "wsluser".into()
}

fn default_install_dir() -> PathBuf {
    "%userprofile%/VMs".into()
}

fn default_cloud_init_path() -> PathBuf {
    "cloud-init.yaml".into()
}

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
        path: PathBuf,
    },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CloudInitInput {
    File {
        #[serde(default = "default_cloud_init_path")]
        path: PathBuf,
    },
    Inline {
        content: String,
    },
}

impl fmt::Display for CloudInitInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudInitInput::File { path } => write!(f, "file: {}", path.display()),
            CloudInitInput::Inline { .. } => write!(f, "inline"),
        }
    }
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Distro { name: default_distro() }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    #[serde(default, rename = "override")]
    pub override_instance: bool,
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default)]
    pub password: Option<String>,

    #[serde(default)]
    pub http_proxy: Option<Url>,
    #[serde(default)]
    pub https_proxy: Option<Url>,
    #[serde(default)]
    pub no_proxy: Option<String>,

    #[serde(default = "default_install_dir")]
    pub install_dir: PathBuf,
    #[serde(default)]
    pub cloud_init: Option<CloudInitInput>,

    #[serde(default)]
    pub image: ImageSource,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootConfig {
    pub profiles: BTreeMap<String, Profile>,
}

#[cfg(test)]
#[path = "../../tests/unit/config/model_tests.rs"]
mod model_tests;
