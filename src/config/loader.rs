use crate::config::{Profile, RootConfig};
use anyhow::Context;
use std::collections::BTreeMap;
use std::{fs, path::Path};

pub fn load_yaml(path: &Path) -> anyhow::Result<RootConfig> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("unable to read config file: {}", path.display()))?;

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
                "invalid yaml in: {}\n- profiles format error: {}\n- single-profile format error: {}",
                path.display(),
                root_err,
                profile_err
            )),
        },
    }
}
