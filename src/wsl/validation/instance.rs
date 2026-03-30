use crate::config::{ImageSource, Instance, SourcePath};
use crate::wsl::helpers::expand_env_vars;

/// Returns the reasons why this instance requires cloud-init but doesn't have it configured.
/// Add new rules here as features require cloud-init support.
pub fn cloud_init_required(instance: &Instance) -> Vec<&'static str> {
    if instance.cloud_init.is_some() || instance.default_cloud_init {
        return vec![];
    }
    let mut reasons: Vec<&'static str> = Vec::new();
    if instance.username.is_some() || instance.password.is_some() {
        reasons.push("username/password set — user account requires cloud-init to be created");
    }
    if instance.proxy.is_some() {
        reasons.push("proxy set — proxy configuration requires cloud-init to be applied");
    }
    reasons
}

pub fn validate_instance(instance: &Instance) -> anyhow::Result<()> {
    validate_image_source(instance)
}

pub fn validate_image_source(instance: &Instance) -> anyhow::Result<()> {
    if let ImageSource::File { path } = &instance.image {
        let local = match path {
            SourcePath::Remote(_) => return Ok(()),
            SourcePath::Local(p) => p,
        };
        let expanded: String = expand_env_vars(&local.to_string_lossy())?;
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
