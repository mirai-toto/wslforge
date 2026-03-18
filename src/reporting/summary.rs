use crate::config::{ImageSource, Instance};
use crate::wsl::helpers::resolve_install_dir;
use log::info;

pub fn log_config_summary(instance_name: &str, instance: &Instance) {
    info!("🧩 Instance: {}", instance_name);
    info!("♻️ Override: {}", instance.override_instance);
    info!("🏷️ Hostname: {}", instance.hostname);
    info!("👤 User: {}", instance.username);
    info!("📦 Install dir: {}", resolved_install_dir_display(instance));
    match &instance.cloud_init {
        Some(source) => info!("☁️ Cloud-init: {}", source),
        None => info!("☁️ Cloud-init: not configured"),
    }

    match &instance.image {
        ImageSource::Distro { name } => {
            info!("🐧 Using WSL distro '{}'", name);
        }
        ImageSource::File { path } => {
            info!("🗂️  Using image file {:?}", path);
        }
    }

    if let Some(proxy) = &instance.http_proxy {
        info!("🌐 HTTP proxy: {}", proxy);
    }
    if let Some(proxy) = &instance.https_proxy {
        info!("🔐 HTTPS proxy: {}", proxy);
    }
}

fn resolved_install_dir_display(instance: &Instance) -> String {
    match resolve_install_dir(&instance.install_dir, &instance.hostname) {
        Ok(path) => path.display().to_string(),
        Err(_) => instance.install_dir.join(&instance.hostname).display().to_string(),
    }
}
