use log::{info, warn};
use std::path::Path;

pub fn store(target_file: &Path, rendered: &str) -> anyhow::Result<()> {
    let target_dir = target_file
        .parent()
        .ok_or_else(|| anyhow::anyhow!("cloud-init target missing parent directory"))?;
    std::fs::create_dir_all(target_dir)?;
    std::fs::write(target_file, rendered)?;
    Ok(())
}

pub fn copy_debug_to_current_dir(hostname: &str, rendered: &str) {
    let debug_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            warn!("☁️ Cloud-init debug copy skipped (cwd error): {err}");
            return;
        }
    };
    let debug_path = debug_dir.join(format!("cloud-init.{}.user-data", hostname));
    if let Err(err) = std::fs::write(&debug_path, rendered) {
        warn!("☁️ Cloud-init debug copy skipped (write error): {}", err);
    } else {
        info!("☁️ Cloud-init debug copy: {}", debug_path.display());
    }
}
