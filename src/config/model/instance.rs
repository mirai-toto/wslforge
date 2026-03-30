use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use url::Url;

use super::cloud_init::CloudInitSource;
use super::image::ImageSource;

fn default_install_dir() -> PathBuf {
    "%userprofile%/VMs".into()
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Proxy {
    #[serde(default)]
    pub http: Option<Url>,
    #[serde(default)]
    pub https: Option<Url>,
    #[serde(default)]
    pub no_proxy: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileTransfer {
    pub src: PathBuf,
    pub dest: String,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScriptConfig {
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub run: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Instance {
    #[serde(default, rename = "override")]
    pub override_instance: bool,
    #[serde(skip)]
    pub name: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,

    #[serde(default)]
    pub proxy: Option<Proxy>,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    #[serde(default)]
    pub files: Vec<FileTransfer>,
    #[serde(default)]
    pub scripts: ScriptConfig,

    #[serde(default = "default_install_dir")]
    pub install_dir: PathBuf,
    #[serde(default)]
    pub cloud_init: Option<CloudInitSource>,
    #[serde(skip)]
    pub default_cloud_init: bool,

    #[serde(default)]
    pub image: ImageSource,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub instances: BTreeMap<String, Instance>,
}
