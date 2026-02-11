use crate::config::{ImageSource, Profile};
use crate::wsl::helpers::path;
use crate::wsl::{CreateEvent, CreateOutcome, CreateReport, EnvironmentEvent, EnvironmentReport};
use log::info;

pub fn log_create_report(report: &CreateReport, hostname: &str) {
    for event in &report.events {
        log_create_event(event, hostname);
    }

    log_create_outcome(report.outcome, hostname);
}

pub fn log_environment_report(report: &EnvironmentReport) {
    for event in &report.events {
        log_environment_event(event);
    }
}

pub fn log_config_summary(profile_name: &str, profile: &Profile) {
    info!("🧩 Profile: {}", profile_name);
    info!("♻️ Override: {}", profile.override_instance);
    info!("🏷️ Hostname: {}", profile.hostname);
    info!("👤 User: {}", profile.username);
    info!("📦 Install dir: {}", expand_install_dir(profile));
    match &profile.cloud_init {
        Some(source) => info!("☁️ Cloud-init: {}", source),
        None => info!("☁️ Cloud-init: not configured"),
    }

    match &profile.image {
        ImageSource::Distro { name } => {
            info!("🐧 Using WSL distro '{}'", name);
        }
        ImageSource::File { path } => {
            info!("🗂️  Using image file {:?}", path);
        }
    }

    if let Some(proxy) = &profile.http_proxy {
        info!("🌐 HTTP proxy: {}", proxy);
    }
    if let Some(proxy) = &profile.https_proxy {
        info!("🔐 HTTPS proxy: {}", proxy);
    }
}

fn expand_install_dir(profile: &Profile) -> String {
    match path::expand_path(&profile.install_dir) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => profile.install_dir.to_string_lossy().into_owned(),
    }
}

fn log_environment_event(event: &EnvironmentEvent) {
    info!("{} {}", event.icon(), event.message());
}

fn log_create_event(event: &CreateEvent, hostname: &str) {
    if let Some(message) = event.message(hostname) {
        info!("{} {}", event.icon(), message);
    }
}

fn log_create_outcome(outcome: CreateOutcome, hostname: &str) {
    info!("{} {}", outcome.icon(), outcome.message(hostname));
}
