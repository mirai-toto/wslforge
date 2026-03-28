use console::style;

use crate::config::{ImageSource, Instance};
use crate::wsl::cloud_init::DEFAULT_CLOUD_INIT_TEMPLATE;
use crate::wsl::helpers::resolve_install_dir;

pub fn log_config_summary(instance_name: &str, instance: &Instance) {
    let image = image_label(&instance.image);

    eprintln!("{}", style(format!("📋 Instance '{instance_name}'")).bold());
    eprintln!(
        "{}",
        style("── Summary ──────────────────────────────────────────").dim()
    );

    eprintln!("  hostname    : {}", style(&instance.hostname).cyan());
    eprintln!(
        "  username    : {}",
        instance
            .username
            .as_deref()
            .map(|v| style(v).cyan().to_string())
            .unwrap_or_else(none_label)
    );
    eprintln!(
        "  password    : {}",
        if instance.password.is_some() {
            style("set").cyan().to_string()
        } else {
            none_label()
        }
    );
    eprintln!("  override    : {}", style(instance.override_instance).cyan());
    eprintln!(
        "  install_dir : {}",
        style(resolved_install_dir_display(instance)).cyan()
    );
    eprintln!("  image       : {}", style(&image).cyan());

    if let Some(proxy) = &instance.proxy {
        if let Some(v) = &proxy.http {
            eprintln!("  proxy http  : {}", style(v.as_ref()).cyan());
        }
        if let Some(v) = &proxy.https {
            eprintln!("  proxy https : {}", style(v.as_ref()).cyan());
        }
        if let Some(v) = &proxy.no_proxy {
            eprintln!("  no proxy    : {}", style(v).cyan());
        }
    } else {
        eprintln!("  proxy       : {}", none_label());
    }

    match &instance.cloud_init {
        Some(ci) => eprintln!("  cloud-init  : {}", style(format!("{ci}")).cyan()),
        None if instance.default_cloud_init => eprintln!(
            "  cloud-init  : {}\n{}",
            style("default (auto-generated):").cyan(),
            style(DEFAULT_CLOUD_INIT_TEMPLATE).dim()
        ),
        None => eprintln!("  cloud-init  : {}", none_label()),
    };

    if !instance.vars.is_empty() {
        eprintln!("  vars        : {}", style(format!("{:?}", instance.vars)).cyan());
    }
    if !instance.files.is_empty() {
        eprintln!(
            "  files       : {}",
            style(format!("{} transfer(s)", instance.files.len())).cyan()
        );
    }
    if !instance.scripts.run.is_empty() {
        eprintln!(
            "  scripts     : {}",
            style(format!("{} script(s)", instance.scripts.run.len())).cyan()
        );
    }
    eprintln!();

    log::debug!(
        target: "wslforge::events",
        "config loaded: instance={instance_name} user={} image={image} override={} cloud-init={}",
        instance.username.as_deref().unwrap_or("(none)"),
        instance.override_instance,
        cloud_init_label(&instance.cloud_init),
    );
}

fn image_label(image: &ImageSource) -> String {
    match image {
        ImageSource::Distro { name } => format!("{name} (distro)"),
        ImageSource::File { path } => format!("{path} (file)"),
    }
}

fn cloud_init_label(cloud_init: &Option<crate::config::CloudInitSource>) -> String {
    match cloud_init {
        Some(source) => source.to_string(),
        None => "not configured".to_string(),
    }
}

fn none_label() -> String {
    style("none").red().to_string()
}

fn resolved_install_dir_display(instance: &Instance) -> String {
    match resolve_install_dir(&instance.install_dir, &instance.hostname) {
        Ok(path) => path.display().to_string(),
        Err(_) => instance.install_dir.join(&instance.hostname).display().to_string(),
    }
}
