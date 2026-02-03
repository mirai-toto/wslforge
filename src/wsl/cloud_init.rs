use crate::config::{AppConfig, CloudInitSource};
use log::info;
use std::path::PathBuf;

pub fn prepare_cloud_init(cfg: &AppConfig) -> anyhow::Result<()> {
    let Some(source) = &cfg.cloud_init else {
        info!("☁️ Cloud-init: not configured");
        return Ok(());
    };

    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    let target_file = target_dir.join(format!("{}.user-data", cfg.hostname));

    info!("☁️ Cloud-init target: {}", target_file.display());

    std::fs::create_dir_all(&target_dir)?;
    match source {
        CloudInitSource::File { path } => {
            let expanded = expand_env_vars(&path.to_string_lossy())?;
            let expanded_path = PathBuf::from(expanded);
            if !expanded_path.exists() {
                anyhow::bail!(
                    "cloud-init user-data file not found: {}",
                    expanded_path.display()
                );
            }
            info!("☁️ Cloud-init source: {}", expanded_path.display());
            std::fs::copy(expanded_path, &target_file)?;
        }
        CloudInitSource::Inline { content } => {
            info!("☁️ Cloud-init source: inline content");
            std::fs::write(&target_file, content)?;
        }
    }
    Ok(())
}

fn expand_env_vars(raw: &str) -> anyhow::Result<String> {
    let percent_expanded = expand_str::expand_string_with_env(raw).map_err(|e| {
        anyhow::anyhow!("environment variable expansion failed: {e}")
    })?;
    let expanded = shellexpand::env(&percent_expanded).map_err(|e| {
        anyhow::anyhow!(
            "environment variable '{}' is not set (from '{}')",
            e.var_name,
            raw
        )
    })?;
    Ok(expanded.into_owned())
}

fn resolve_userprofile_dir() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(path));
    }
    anyhow::bail!("USERPROFILE is not set; cannot place cloud-init user-data")
}
