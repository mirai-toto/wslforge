use console::style;

use crate::config::{ImageSource, Instance};
use crate::wsl::helpers::resolve_install_dir;

pub fn log_config_summary(instance_name: &str, instance: &Instance) {
    let image = image_label(&instance.image);
    let cloud_init = cloud_init_label(&instance.cloud_init);

    eprintln!("{}", style(format!("📋 Instance '{instance_name}'")).bold());
    let field = |key: &str, val: &str| eprintln!("   {}  {}", style(format!("{key:<12}")).dim(), val);
    field("hostname", &instance.hostname);
    field("user", &instance.username);
    field("override", &instance.override_instance.to_string());
    field("install dir", &resolved_install_dir_display(instance));
    field("image", &image);
    field("cloud-init", &cloud_init);
    if let Some(proxy) = &instance.proxy {
        if let Some(v) = &proxy.http {
            field("proxy http", v.as_ref());
        }
        if let Some(v) = &proxy.https {
            field("proxy https", v.as_ref());
        }
        if let Some(v) = &proxy.no_proxy {
            field("no proxy", v);
        }
    }
    if !instance.vars.is_empty() {
        field("vars", &format!("{:?}", instance.vars));
    }
    if !instance.files.is_empty() {
        field("files", &format!("{} transfer(s)", instance.files.len()));
    }
    eprintln!();

    log::debug!(
        "config loaded: instance={instance_name} user={} image={image} override={} cloud-init={cloud_init}",
        instance.username,
        instance.override_instance,
    );
}

fn image_label(image: &ImageSource) -> String {
    match image {
        ImageSource::Distro { name } => format!("{name} (distro)"),
        ImageSource::File { path } => format!("{} (file)", path.display()),
    }
}

fn cloud_init_label(cloud_init: &Option<crate::config::CloudInitSource>) -> String {
    match cloud_init {
        Some(source) => source.to_string(),
        None => "not configured".to_string(),
    }
}

fn resolved_install_dir_display(instance: &Instance) -> String {
    match resolve_install_dir(&instance.install_dir, &instance.hostname) {
        Ok(path) => path.display().to_string(),
        Err(_) => instance.install_dir.join(&instance.hostname).display().to_string(),
    }
}
