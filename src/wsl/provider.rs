use crate::wsl::engine::api::ApiEngine;
use crate::wsl::engine::cli::CliEngine;
use crate::wsl::engine::{CreateOutcome, WslEngine};
use log::info;

pub enum EngineKind {
    Cli,
    Api,
}

pub struct WslProvider {
    engine: Box<dyn WslEngine>,
}

impl WslProvider {
    pub fn new(kind: EngineKind) -> Self {
        let engine: Box<dyn WslEngine> = match kind {
            EngineKind::Cli => Box::new(CliEngine::new()),
            EngineKind::Api => Box::new(ApiEngine::new()),
        };
        Self { engine }
    }

    pub fn instance_exists(&self, name: &str) -> anyhow::Result<bool> {
        info!("🔍 Checking if WSL instance '{}' exists...", name);
        let exists = self.engine.instance_exists(name)?;
        if exists {
            info!("✅ WSL instance '{}' exists.", name);
        } else {
            info!("ℹ️ WSL instance '{}' does not exist.", name);
        }
        Ok(exists)
    }

    pub fn delete_instance(&self, name: &str) -> anyhow::Result<()> {
        info!("🧹 Deleting existing WSL instance '{}'", name);
        if !self.engine.instance_exists(name)? {
            info!(
                "ℹ️ WSL instance '{}' does not exist. Skipping delete.",
                name
            );
            return Ok(());
        }
        self.engine.delete_instance(name)?;
        info!("✅ WSL instance '{}' deleted successfully.", name);
        Ok(())
    }

    pub fn create_from_file(
        &self,
        name: &str,
        install_dir: &std::path::Path,
        rootfs_tar: &std::path::Path,
    ) -> anyhow::Result<CreateOutcome> {
        self.engine
            .create_from_file(name, install_dir, rootfs_tar)?;
        Ok(CreateOutcome::Created)
    }

    pub fn create_from_distro(
        &self,
        distro_name: &str,
        hostname: &str,
    ) -> anyhow::Result<CreateOutcome> {
        self.engine.create_from_distro(distro_name, hostname)?;
        Ok(CreateOutcome::Created)
    }
}
