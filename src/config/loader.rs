use crate::config::{Profile, RootConfig};
use anyhow::Context;
use std::collections::BTreeMap;
use std::{fs, path::Path};

fn format_yaml_error(path: &Path, err: &serde_yaml::Error) -> String {
    if let Some(loc) = err.location() {
        format!("{}:{}:{}: {}", path.display(), loc.line(), loc.column(), err)
    } else {
        format!("{}: {}", path.display(), err)
    }
}

pub fn load_yaml(path: &Path) -> anyhow::Result<RootConfig> {
    let raw = fs::read_to_string(path).with_context(|| format!("unable to read config file: {}", path.display()))?;

    match serde_yaml::from_str::<RootConfig>(&raw) {
        Ok(cfg) => Ok(cfg),
        Err(root_err) => match serde_yaml::from_str::<Profile>(&raw) {
            Ok(profile) => {
                let mut profiles = BTreeMap::new();
                let name = profile.hostname.clone();
                profiles.insert(name, profile);
                Ok(RootConfig { profiles })
            }
            Err(profile_err) => Err(anyhow::anyhow!(
                "invalid yaml\n- profiles format error: {}\n- single-profile format error: {}\n\nExpected either:\n- profiles:\n    <name>:\n      <profile>\n- or a single profile object at the root",
                format_yaml_error(path, &root_err),
                format_yaml_error(path, &profile_err)
            )),
        },
    }
}
