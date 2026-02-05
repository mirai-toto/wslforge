use super::helpers::{expand_env_vars, hash_password_sha512, resolve_userprofile_dir};
use crate::config::{AppConfig, CloudInitSource};
use log::{debug, info, warn};
use minijinja::Environment;
use std::path::PathBuf;

pub fn prepare_cloud_init(
    cfg: &AppConfig,
    dry_run: bool,
    debug: bool,
) -> anyhow::Result<()> {
    let Some(source) = &cfg.cloud_init else {
        info!("☁️ Cloud-init: not configured");
        return Ok(());
    };

    let target_file = cloud_init_target(&cfg.hostname)?;
    info!("☁️ Cloud-init target: {}", target_file.display());

    let raw = load_cloud_init_source(source)?;
    let rendered = render_cloud_init(&raw, cfg)?;
    debug!("☁️ Cloud-init rendered:\n{}", rendered);
    write_cloud_init(
        &target_file,
        &rendered,
        &cfg.hostname,
        dry_run,
        debug,
    )?;
    Ok(())
}

// Determine the target path for the cloud-init user-data file based on the hostname.
fn cloud_init_target(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    std::fs::create_dir_all(&target_dir)?;
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

fn load_cloud_init_source(source: &CloudInitSource) -> anyhow::Result<String> {
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
            std::fs::read_to_string(expanded_path).map_err(Into::into)
        }
        CloudInitSource::Inline { content } => {
            info!("☁️ Cloud-init source: inline content");
            Ok(content.to_string())
        }
    }
}

fn write_cloud_init(
    target_file: &PathBuf,
    rendered: &str,
    hostname: &str,
    dry_run: bool,
    debug: bool,
) -> anyhow::Result<()> {
    if !dry_run {
        std::fs::write(target_file, rendered)?;
    }
    if debug {
        debug_cloud_init(rendered, hostname);
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

fn debug_cloud_init(rendered: &str, hostname: &str) {
    let debug_path = match std::env::current_dir() {
        Ok(dir) => dir.join(format!("cloud-init.{}.user-data", hostname)),
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
