use crate::config::{ImageSource, Profile};
use crate::wsl::engine::CreateOutcome;
use crate::wsl::helpers::expand_env_vars;
use log::info;

pub fn log_create_outcome(outcome: CreateOutcome, hostname: &str) {
    match outcome {
        CreateOutcome::Created => {
            info!("✅ WSL instance '{}' created successfully.", hostname);
        }
        CreateOutcome::AlreadyExists => {
            info!("ℹ️ WSL instance '{}' already exists.", hostname);
        }
        CreateOutcome::Skipped => {
            info!("ℹ️ WSL instance '{}' was skipped.", hostname);
        }
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
    expand_env_vars(&profile.install_dir.to_string_lossy())
        .unwrap_or_else(|_| profile.install_dir.to_string_lossy().into_owned())
}
