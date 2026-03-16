use crate::config::{ImageSource, Profile};
use crate::wsl::helpers::path;
use log::info;

pub fn log_config_summary(profile_name: &str, profile: &Profile) {
    info!("🧩 Profile: {}", profile_name);
    info!("♻️ Override: {}", profile.override_instance);
    info!("🏷️ Hostname: {}", profile.hostname);
    info!("👤 User: {}", profile.username);
    info!("📦 Install dir: {}", resolved_install_dir_display(profile));
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

fn resolved_install_dir_display(profile: &Profile) -> String {
    match path::resolve_install_dir(&profile.install_dir, &profile.hostname) {
        Ok(path) => path.display().to_string(),
        Err(_) => profile.install_dir.join(&profile.hostname).display().to_string(),
    }
}
