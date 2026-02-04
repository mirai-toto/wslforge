// NOTE: We only *read* this config from YAML, but we also derive `Serialize` so we can pass
// it into the cloud-init template renderer (minijinja) as `cfg`.
// - Nothing writes this config back to disk.
// - `skip_serializing_if` on `Option<T>` makes `None` act like "missing" in templates, so
//   `| default('...')` works as expected.
// - `password` is optional; we hash it when rendering cloud-init templates.
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Distro {
            name: default_distro(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppConfig {
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,

    #[serde(default = "default_install_dir")]
    pub install_dir: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_init: Option<CloudInitSource>,

    #[serde(default)]
    pub image: ImageSource,
}
