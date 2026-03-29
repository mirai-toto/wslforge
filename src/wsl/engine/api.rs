use crate::wsl::engine::{FileAttrs, WslEngine};

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
        _attrs: FileAttrs<'_>,
        _shell: &str,
    ) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn write_dir(
        &self,
        _instance_name: &str,
        _src: &std::path::Path,
        _dest: &str,
        _attrs: FileAttrs<'_>,
        _shell: &str,
    ) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn run_script(&self, _instance_name: &str, _script: &str, _shell: &str) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }

    fn wait_for_provisioning(&self, _instance_name: &str) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }
}
