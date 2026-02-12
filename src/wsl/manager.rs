use crate::config::Profile;
use crate::wsl::engine::api::ApiEngine;
use crate::wsl::engine::cli::CliEngine;
use crate::wsl::engine::{EngineKind, WslEngine};
use crate::wsl::services::CreateInstanceService;
use crate::wsl::validation::{config, environment};
use crate::wsl::{CreateReport, EnvironmentReport, ExecutionOptions};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
}

impl WslManager {
    pub fn new() -> Self {
        Self {
            engine: build_engine(EngineKind::Cli),
        }
    }

    pub fn with_engine(kind: EngineKind) -> Self {
        Self {
            engine: build_engine(kind),
        }
    }

    pub fn validate_environment(&self, options: ExecutionOptions) -> anyhow::Result<EnvironmentReport> {
        environment::validate_environment(options)
    }

    pub fn validate_profile_config(&self, profile: &Profile) -> anyhow::Result<()> {
        config::validate_profile(profile)
    }

    pub fn create_instance(
        &self,
        _profile_name: &str,
        profile: &Profile,
        options: ExecutionOptions,
    ) -> anyhow::Result<CreateReport> {
        self.validate_profile_config(profile)?;
        CreateInstanceService::new(self.engine.as_ref(), options).execute(profile)
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new()
    }
}

fn build_engine(kind: EngineKind) -> Box<dyn WslEngine> {
    match kind {
        EngineKind::Cli => Box::new(CliEngine::new()),
        EngineKind::Api => Box::new(ApiEngine::new()),
    }
}
