use serde::Deserialize;
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

fn default_cloud_init() -> PathBuf {
    "cloud-init.yaml".into()
}

fn default_distro() -> String {
    "Ubuntu".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageSource {
    Distro {
        #[serde(default = "default_distro")]
        name: String,
    },
    File {
        path: PathBuf,
    },
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Distro {
            name: default_distro(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
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
    #[serde(default = "default_cloud_init")]
    pub cloud_init: PathBuf,

    #[serde(default)]
    pub image: ImageSource,
}
