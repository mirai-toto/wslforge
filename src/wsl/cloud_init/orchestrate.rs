use crate::config::Profile;
use crate::wsl::helpers::userprofile::resolve_userprofile_dir;
use log::{debug, info};
use std::path::PathBuf;

use super::{copy_debug_to_current_dir, load::load_cloud_init_source, render, store};

pub fn cloud_init_target_file(hostname: &str) -> anyhow::Result<PathBuf> {
    let userprofile = resolve_userprofile_dir()?;
    let target_dir = userprofile.join(".cloud-init");
    Ok(target_dir.join(format!("{}.user-data", hostname)))
}

pub fn prepare_cloud_init(profile: &Profile, dry_run: bool, debug: bool) -> anyhow::Result<()> {
    let Some(source) = &profile.cloud_init else {
        info!("☁️ Cloud-init: not configured");
        return Ok(());
    };

    let raw = load_cloud_init_source(source)?;
    let rendered = render(&raw, profile)?;
    debug!("☁️ Cloud-init rendered:\n{}", rendered);

    let target_file = cloud_init_target_file(&profile.hostname)?;
    if dry_run {
        info!(
            "🧪 Dry run: cloud-init target would be created at: {}",
            target_file.display()
        );
        return Ok(());
    }

    store(&target_file, &rendered)?;
    if debug {
        copy_debug_to_current_dir(&profile.hostname, &rendered);
    }
    info!("☁️ Cloud-init target: {}", target_file.display());
    Ok(())
}
