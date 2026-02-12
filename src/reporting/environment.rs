use crate::wsl::{EnvironmentEvent, EnvironmentReport};
use log::info;

pub fn log_environment_report(report: &EnvironmentReport) {
    for event in &report.events {
        log_environment_event(event);
    }
}

fn log_environment_event(event: &EnvironmentEvent) {
    info!("{} {}", environment_event_icon(event), environment_event_message(event));
}

fn environment_event_icon(event: &EnvironmentEvent) -> &'static str {
    match event {
        EnvironmentEvent::WslUpdateDryRun => "🧪",
        EnvironmentEvent::WslInstalled
        | EnvironmentEvent::WslUpdateCompleted
        | EnvironmentEvent::WindowsFeatureEnabled(_) => "✅",
    }
}

fn environment_event_message(event: &EnvironmentEvent) -> String {
    match event {
        EnvironmentEvent::WslInstalled => "WSL is installed".to_string(),
        EnvironmentEvent::WslUpdateDryRun => "Dry run: WSL update would be performed".to_string(),
        EnvironmentEvent::WslUpdateCompleted => "WSL update completed".to_string(),
        EnvironmentEvent::WindowsFeatureEnabled(name) => format!("{name} is enabled"),
    }
}
