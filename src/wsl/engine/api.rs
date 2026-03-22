use crate::wsl::engine::WslEngine;

/// Placeholder for a future WSL API-based engine. All methods are unimplemented.
pub struct ApiEngine;

impl WslEngine for ApiEngine {
    fn status(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn update(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn list_online_distros(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn instance_exists(&self, _name: &str) -> anyhow::Result<bool> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn delete_instance(&self, _name: &str) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn create_from_file(
        &self,
        _name: &str,
        _install_dir: &std::path::Path,
        _rootfs_tar: &std::path::Path,
    ) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn create_from_distro(&self, _distro_name: &str, _instance_name: &str) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn write_file(
        &self,
        _instance_name: &str,
        _dest: &str,
        _content: &[u8],
        _owner: Option<&str>,
        _mode: Option<&str>,
    ) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }
}
