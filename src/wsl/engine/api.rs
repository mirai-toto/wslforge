use crate::wsl::engine::WslEngine;

pub struct ApiEngine;

impl ApiEngine {
    pub fn new() -> Self {
        Self
    }
}

impl WslEngine for ApiEngine {
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

    fn create_from_distro(&self, _distro_name: &str, _name: &str) -> anyhow::Result<()> {
        anyhow::bail!("WSL API engine is not implemented yet")
    }
}
