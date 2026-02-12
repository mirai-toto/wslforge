use crate::config::Profile;
use crate::wsl::engine::WslEngine;
use crate::wsl::services::CreateInstanceService;
use crate::wsl::validation::{config, environment};
use crate::wsl::{CreateReport, EnvironmentReport, ExecutionOptions};

pub struct WslManager {
    engine: Box<dyn WslEngine>,
}

impl WslManager {
    pub fn new(engine: Box<dyn WslEngine>) -> Self {
        Self { engine }
    }

    pub fn validate_environment(&self) -> anyhow::Result<EnvironmentReport> {
        let events = environment::check_environment()?;
        Ok(EnvironmentReport { events })
    }

    pub fn create_instance(
        &self,
        _profile_name: &str,
        profile: &Profile,
        options: ExecutionOptions,
    ) -> anyhow::Result<CreateReport> {
        config::validate_profile(profile)?;
        CreateInstanceService::new(self.engine.as_ref(), options).execute(profile)
    }
}
