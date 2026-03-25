use crate::wsl::helpers::expand_env_vars;
use std::path::{Path, PathBuf};

pub(super) fn load_cloud_init_source(path: &Path) -> anyhow::Result<String> {
    let expanded: String = expand_env_vars(&path.to_string_lossy())?;
    let expanded_path: PathBuf = PathBuf::from(expanded);
    if !expanded_path.exists() {
        anyhow::bail!("cloud-init user-data file not found: {}", expanded_path.display());
    }
    Ok(std::fs::read_to_string(&expanded_path)?)
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/load_tests.rs"]
mod load_tests;
