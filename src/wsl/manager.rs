use crate::config::Profile;
use crate::wsl::engine::api::ApiEngine;
use crate::wsl::engine::cli::CliEngine;
use crate::wsl::engine::{EngineKind, WslEngine};
use crate::wsl::use_cases::CreateInstanceService;
use crate::wsl::validation::{config, environment};
use crate::wsl::{CreateReport, EnvironmentReport};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
    dry_run: bool,
    debug: bool,
}

impl WslManager {
    pub fn new(dry_run: bool, debug: bool) -> Self {
        Self {
            engine: build_engine(EngineKind::Cli),
            dry_run,
            debug,
        }
    }

    pub fn with_engine(kind: EngineKind, dry_run: bool, debug: bool) -> Self {
        Self {
            engine: build_engine(kind),
            dry_run,
            debug,
        }
    }

    pub fn validate_environment(&self) -> anyhow::Result<EnvironmentReport> {
        environment::validate_environment(self.dry_run)
    }

    pub fn validate_profile_config(&self, profile: &Profile) -> anyhow::Result<()> {
        config::validate_profile(profile)
    }

    pub fn create_instance(&self, _profile_name: &str, profile: &Profile) -> anyhow::Result<CreateReport> {
        self.validate_profile_config(profile)?;
        CreateInstanceService::new(self.engine.as_ref(), self.dry_run, self.debug).execute(profile)
    }
}

impl Default for WslManager {
    fn default() -> Self {
        Self::new(false, false)
    }
}

fn build_engine(kind: EngineKind) -> Box<dyn WslEngine> {
    match kind {
        EngineKind::Cli => Box::new(CliEngine::new()),
        EngineKind::Api => Box::new(ApiEngine::new()),
    }
}
