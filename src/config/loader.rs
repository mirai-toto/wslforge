use crate::config::Profile;
use anyhow::Context;
use std::{fs, path::Path};

pub fn load_yaml(path: &Path) -> anyhow::Result<Profile> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("unable to read config file: {}", path.display()))?;
    let cfg = serde_yaml::from_str(&raw)
        .with_context(|| format!("invalid yaml in: {}", path.display()))?;
    Ok(cfg)
}
