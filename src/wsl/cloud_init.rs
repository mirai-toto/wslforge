use super::env::{expand_env_vars, resolve_userprofile_dir};
use crate::config::{AppConfig, CloudInitSource};
use log::{info, warn};
use minijinja::Environment;
use sha_crypt::{sha512_simple, Sha512Params, ROUNDS_DEFAULT};
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
            let raw = std::fs::read_to_string(expanded_path)?;
            let rendered = render_cloud_init(&raw, cfg)?;
            std::fs::write(&target_file, &rendered)?;
            write_debug_copy(&rendered, cfg);
        }
        CloudInitSource::Inline { content } => {
            info!("☁️ Cloud-init source: inline content");
            let rendered = render_cloud_init(content, cfg)?;
            std::fs::write(&target_file, &rendered)?;
            write_debug_copy(&rendered, cfg);
        }
    }
    Ok(())
}

fn render_cloud_init(raw: &str, cfg: &AppConfig) -> anyhow::Result<String> {
    let mut env = Environment::new();
    env.add_template("cloud-init.user-data", raw)
        .map_err(|e| anyhow::anyhow!("cloud-init template parse error: {e}"))?;

    let template = env
        .get_template("cloud-init.user-data")
        .map_err(|e| anyhow::anyhow!("cloud-init template load error: {e}"))?;

    let password_hash = match cfg.password.as_deref() {
        Some(password) => Some(hash_password_sha512(password)?),
        None => None,
    };

    template
        .render(minijinja::context! { cfg => cfg, password_hash => password_hash })
        .map_err(|e| anyhow::anyhow!("cloud-init template render error: {e}"))
}

fn hash_password_sha512(password: &str) -> anyhow::Result<String> {
    let params = Sha512Params::new(ROUNDS_DEFAULT)
        .map_err(|e| anyhow::anyhow!("invalid sha512-crypt params: {e:?}"))?;
    sha512_simple(password, &params).map_err(|e| anyhow::anyhow!("password hashing failed: {e:?}"))
}

fn write_debug_copy(rendered: &str, cfg: &AppConfig) {
    let debug_path = match std::env::current_dir() {
        Ok(dir) => dir.join(format!("cloud-init.{}.user-data", cfg.hostname)),
        Err(err) => {
            warn!("☁️ Cloud-init debug copy skipped (cwd error): {err}");
            return;
        }
    };

    if let Err(err) = std::fs::write(&debug_path, rendered) {
        warn!("☁️ Cloud-init debug copy skipped (write error): {}", err);
    } else {
        info!("☁️ Cloud-init debug copy: {}", debug_path.display());
    }
}
