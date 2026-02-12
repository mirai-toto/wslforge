use std::collections::BTreeMap;

use crate::config::{Profile, RootConfig};
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

    pub fn validate_environment(&self) -> anyhow::Result<EnvironmentReport> {
        let events = environment::check_environment()?;
        Ok(EnvironmentReport { events })
    }

    pub fn prepare_environment(&self, dry_run: bool) -> anyhow::Result<EnvironmentReport> {
        let mut report = self.validate_environment()?;
        let update_event = maintenance::environment::update_wsl_version(dry_run)?;
        report.events.push(update_event);
        Ok(report)
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

    pub fn apply_config(
        &self,
        root: &RootConfig,
        options: ExecutionOptions,
    ) -> anyhow::Result<(EnvironmentReport, BTreeMap<String, CreateReport>)> {
        let environment_report = self.prepare_environment(options.dry_run)?;

        let mut create_reports_by_profile = BTreeMap::new();
        for (profile_name, profile) in &root.profiles {
            let report = self.create_instance(profile_name, profile, options)?;
            create_reports_by_profile.insert(profile_name.clone(), report);
        }

        Ok((environment_report, create_reports_by_profile))
    }
}
