use crate::config::{ImageSource, Profile};
use log::info;

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
    let raw = profile.install_dir.to_string_lossy();
    let percent_expanded = match expand_str::expand_string_with_env(&raw) {
        Ok(value) => value,
        Err(_) => return raw.into_owned(),
    };
    match shellexpand::env(&percent_expanded) {
        Ok(value) => value.into_owned(),
        Err(_) => raw.into_owned(),
    }
}
