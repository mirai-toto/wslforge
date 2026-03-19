use crate::config::{ImageSource, Instance};
use crate::wsl::helpers::expand_env_vars;

pub fn validate_instance(instance: &Instance) -> anyhow::Result<()> {
    validate_image_source(instance)
}

pub fn validate_image_source(instance: &Instance) -> anyhow::Result<()> {
    if let ImageSource::File { path } = &instance.image {
        let expanded: String = expand_env_vars(&path.to_string_lossy())?;
        let expanded_path: std::path::PathBuf = std::path::PathBuf::from(expanded);
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
    const VALID_EXTENSIONS: &[&str] = &[".tar", ".tar.gz", ".tgz", ".tar.xz"];
    let name: String = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
    VALID_EXTENSIONS.iter().any(|ext| name.ends_with(ext))
}
