pub mod api;
pub mod cli;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Cli,
    Api,
}

pub trait WslEngine {
    fn status(&self) -> anyhow::Result<std::process::Output>;
    fn update(&self) -> anyhow::Result<std::process::Output>;
    fn list_online_distros(&self) -> anyhow::Result<std::process::Output>;

    fn instance_exists(&self, name: &str) -> anyhow::Result<bool>;
    fn delete_instance(&self, name: &str) -> anyhow::Result<()>;
    fn create_from_file(
        &self,
        name: &str,
        install_dir: &std::path::Path,
        rootfs_tar: &std::path::Path,
    ) -> anyhow::Result<()>;
    fn create_from_distro(&self, distro_name: &str, instance_name: &str) -> anyhow::Result<()>;
}
