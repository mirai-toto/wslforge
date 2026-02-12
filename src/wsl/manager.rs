use crate::config::Profile;
use crate::wsl::engine::WslEngine;
use crate::wsl::maintenance;
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

    pub fn validate_environment(&self, options: ExecutionOptions) -> anyhow::Result<EnvironmentReport> {
        let mut events = environment::check_environment()?;
        events.push(maintenance::environment::update_wsl_version(options.dry_run)?);
        Ok(EnvironmentReport { events })
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
