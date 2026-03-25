use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
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

#[derive(Debug, Clone)]
pub enum SourcePath {
    Local(PathBuf),
    Remote(Url),
}

impl<'de> Deserialize<'de> for SourcePath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if let Ok(url) = Url::parse(&s) {
            if matches!(url.scheme(), "http" | "https") {
                return Ok(SourcePath::Remote(url));
            }
        }
        Ok(SourcePath::Local(PathBuf::from(s)))
    }
}

impl fmt::Display for SourcePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourcePath::Local(p) => write!(f, "{}", p.display()),
            SourcePath::Remote(u) => write!(f, "{u}"),
        }
    }
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
    pub mode: Option<String>,
}

impl Default for ImageSource {
    fn default() -> Self {
        ImageSource::Distro { name: default_distro() }
    }
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
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_username")]
    pub username: String,
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

    #[serde(default)]
    pub image: ImageSource,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub instances: BTreeMap<String, Instance>,
}

#[cfg(test)]
#[path = "../../tests/unit/config/model_tests.rs"]
mod model_tests;
