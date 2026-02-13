use std::path::{Path, PathBuf};

// Expands env vars, supporting both %VAR% and $VAR styles.
pub(crate) fn expand_env_vars(raw: &str) -> anyhow::Result<String> {
    let percent_expanded = expand_str::expand_string_with_env(raw)
        .map_err(|e| anyhow::anyhow!("environment variable expansion failed: {e}"))?;
    let expanded = shellexpand::env(&percent_expanded)
        .map_err(|e| anyhow::anyhow!("environment variable '{}' is not set (from '{}')", e.var_name, raw))?;
    Ok(expanded.into_owned())
}

pub(crate) fn expand_path(raw: &Path) -> anyhow::Result<PathBuf> {
    let expanded = expand_env_vars(&raw.to_string_lossy())?;
    Ok(PathBuf::from(expanded))
}

pub(crate) fn resolve_install_dir(install_dir: &Path, hostname: &str) -> anyhow::Result<PathBuf> {
    let expanded = expand_path(install_dir)?;
    Ok(expanded.join(hostname))
}
