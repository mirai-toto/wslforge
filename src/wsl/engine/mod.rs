pub mod api;
pub mod cli;
mod script;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineKind {
    Cli,
    Api,
}

pub struct FileAttrs<'a> {
    pub owner: Option<&'a str>,
    pub group: Option<&'a str>,
    pub mode: Option<&'a str>,
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
    fn write_file(
        &self,
        instance_name: &str,
        dest: &str,
        content: &[u8],
        attrs: FileAttrs<'_>,
        shell: &str,
    ) -> anyhow::Result<()>;
    fn write_dir(
        &self,
        instance_name: &str,
        src: &std::path::Path,
        dest: &str,
        attrs: FileAttrs<'_>,
        shell: &str,
    ) -> anyhow::Result<()>;
    fn run_script(&self, instance_name: &str, script: &str, shell: &str) -> anyhow::Result<()>;
    fn wait_for_provisioning(
        &self,
        instance_name: &str,
        timeout_secs: u64,
        on_status: &dyn Fn(String),
    ) -> anyhow::Result<String>;
}
