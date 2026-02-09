use super::helpers::resolve_userprofile_dir;
use log::{info, warn};
use std::path::{Path, PathBuf};

pub fn store(hostname: &str, rendered: &str, dry_run: bool, debug: bool) -> anyhow::Result<PathBuf> {
    let target_file = create_cloud_init_target(hostname, dry_run)?;
    if !dry_run {
        std::fs::write(&target_file, rendered)?;
    }
    if debug {
        debug_cloud_init(rendered, hostname);
    }
    Ok(target_file)
}

// Determine the target path for the cloud-init user-data file based on the hostname.
fn create_cloud_init_target(hostname: &str, dry_run: bool) -> anyhow::Result<PathBuf> {
    if dry_run {
        info!("🧪 Dry run: cloud-init target would be created at: {}", hostname);
        return Ok(PathBuf::from(format!("{}.user-data", hostname)));
    }
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    std::fs::create_dir_all(&target_dir)?;
    Ok(target_dir.join(format!("{}.user-data", hostname)))
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
