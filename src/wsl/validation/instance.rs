use crate::config::{ImageSource, Instance};
use crate::wsl::helpers::path::expand_env_vars;

pub fn validate_instance(instance: &Instance) -> anyhow::Result<()> {
    validate_image_source(instance)
}

pub fn validate_image_source(instance: &Instance) -> anyhow::Result<()> {
    if let ImageSource::File { path } = &instance.image {
        let expanded = expand_env_vars(&path.to_string_lossy())?;
        let expanded_path = std::path::PathBuf::from(expanded);
        if !expanded_path.exists() {
            anyhow::bail!("image file not found: {}", expanded_path.display());
        }
        if !is_likely_rootfs_archive(&expanded_path) {
            anyhow::bail!(
                "image file must be one of: .tar, .tar.gz, .tgz, .tar.xz; got: {}",
                expanded_path.display()
            );
        }
    }
    Ok(())
}

fn is_likely_rootfs_archive(path: &std::path::Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
    name.ends_with(".tar") || name.ends_with(".tar.gz") || name.ends_with(".tgz") || name.ends_with(".tar.xz")
}
