pub mod api;
pub mod cli;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateOutcome {
    Created,
    AlreadyExists,
    Skipped,
}

pub trait WslEngine {
    fn instance_exists(&self, name: &str) -> anyhow::Result<bool>;
    fn delete_instance(&self, name: &str) -> anyhow::Result<()>;
    fn create_from_file(
        &self,
        name: &str,
        install_dir: &std::path::Path,
        rootfs_tar: &std::path::Path,
    ) -> anyhow::Result<()>;
    fn create_from_distro(&self, distro_name: &str, name: &str) -> anyhow::Result<()>;
}
