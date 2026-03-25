use std::collections::BTreeMap;
use std::path::PathBuf;

use console::style;
use dialoguer::{Confirm, Input, Password};
use url::Url;

use crate::config::{CloudInitSource, Config, ImageSource, Instance, Proxy};

pub fn run() -> anyhow::Result<Config> {
    eprintln!(
        "{}",
        style("⚒️  No config file found. Let's configure your WSL instance.").bold()
    );
    eprintln!();

    let (hostname, instance) = prompt_instance()?;

    eprintln!();
    print_summary(&hostname, &instance);

    let confirmed = Confirm::new()
        .with_prompt(style("🚀 ready to provision — proceed?").cyan().bold().to_string())
        .default(true)
        .interact()?;

    if !confirmed {
        eprintln!("{}", style("Aborted.").yellow());
        std::process::exit(0);
    }

    Ok(Config {
        instances: BTreeMap::from([(hostname, instance)]),
    })
}

fn print_summary(hostname: &str, instance: &Instance) {
    eprintln!(
        "{}",
        style("── Summary ──────────────────────────────────────────").dim()
    );
    eprintln!("  hostname    : {}", style(hostname).cyan());
    eprintln!("  username    : {}", style(&instance.username).cyan());
    eprintln!(
        "  password    : {}",
        style(if instance.password.is_some() { "set" } else { "none" }).cyan()
    );
    eprintln!("  override    : {}", style(instance.override_instance).cyan());
    eprintln!("  install_dir : {}", style(instance.install_dir.display()).cyan());
    eprintln!(
        "  image       : {}",
        style(match &instance.image {
            ImageSource::Distro { name } => format!("distro: {name}"),
            ImageSource::File { path } => format!("file: {path}"),
        })
        .cyan()
    );
    eprintln!(
        "  cloud-init  : {}",
        style(match &instance.cloud_init {
            Some(ci) => format!("{ci}"),
            None => "none".into(),
        })
        .cyan()
    );
    eprintln!(
        "  proxy       : {}",
        style(if instance.proxy.is_some() { "configured" } else { "none" }).cyan()
    );
    eprintln!();
}

fn prompt_instance() -> anyhow::Result<(String, Instance)> {
    eprintln!(
        "{}",
        style("── Instance ─────────────────────────────────────────").dim()
    );

    let hostname: String = Input::new()
        .with_prompt(style("🏷️  hostname").cyan().bold().to_string())
        .default("UbuntuWSL".into())
        .interact_text()?;

    let username: String = Input::new()
        .with_prompt(style("👤 username").cyan().bold().to_string())
        .default("wsluser".into())
        .interact_text()?;

    let password: String = Password::new()
        .with_prompt(style("🔑 password (blank to skip)").cyan().bold().to_string())
        .with_confirmation(
            style("🔑 confirm password").cyan().bold().to_string(),
            "passwords do not match, please try again",
        )
        .allow_empty_password(true)
        .interact()?;

    let override_instance: bool = Confirm::new()
        .with_prompt(style("♻️  override existing instance?").cyan().bold().to_string())
        .default(false)
        .interact()?;

    eprintln!(
        "{}",
        style("── Proxy ────────────────────────────────────────────").dim()
    );

    let proxy = if Confirm::new()
        .with_prompt(style("🌐 configure proxy?").cyan().bold().to_string())
        .default(false)
        .interact()?
    {
        prompt_proxy()?
    } else {
        None
    };

    eprintln!(
        "{}",
        style("── Cloud-init ───────────────────────────────────────").dim()
    );

    let cloud_init = prompt_cloud_init()?;

    eprintln!(
        "{}",
        style("── Image ────────────────────────────────────────────").dim()
    );

    let image = prompt_image()?;

    let instance = Instance {
        override_instance,
        hostname: hostname.clone(),
        username,
        password: if password.is_empty() { None } else { Some(password) },
        proxy,
        vars: Default::default(),
        files: vec![],
        scripts: Default::default(),
        install_dir: PathBuf::from("%userprofile%/VMs"),
        cloud_init,
        image,
    };

    Ok((hostname, instance))
}

fn prompt_cloud_init() -> anyhow::Result<Option<CloudInitSource>> {
    let path: String = Input::new()
        .with_prompt(
            style("☁️  cloud-init file path (blank to skip)")
                .cyan()
                .bold()
                .to_string(),
        )
        .allow_empty(true)
        .interact_text()?;

    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(CloudInitSource::File {
            path: PathBuf::from(path),
        }))
    }
}

fn prompt_image() -> anyhow::Result<ImageSource> {
    let name: String = Input::new()
        .with_prompt(style("🐧 distro name (e.g. Ubuntu, Debian)").cyan().bold().to_string())
        .default("Ubuntu".into())
        .interact_text()?;

    let use_file: bool = Confirm::new()
        .with_prompt(style("🗂️  use a rootfs file or URL instead?").cyan().bold().to_string())
        .default(false)
        .interact()?;

    if use_file {
        let path: String = Input::new()
            .with_prompt(style("🗂️  rootfs path or URL").cyan().bold().to_string())
            .interact_text()?;
        Ok(ImageSource::File {
            path: path.parse().unwrap(),
        })
    } else {
        Ok(ImageSource::Distro { name })
    }
}

fn prompt_proxy() -> anyhow::Result<Option<Proxy>> {
    let http: String = Input::new()
        .with_prompt(style("🌐 proxy http (blank to skip)").cyan().bold().to_string())
        .allow_empty(true)
        .interact_text()?;

    if http.is_empty() {
        return Ok(None);
    }

    let http_url = Url::parse(&http).map_err(|e| anyhow::anyhow!("invalid proxy http URL: {e}"))?;

    let https: String = Input::new()
        .with_prompt(style("🔒 proxy https (blank to skip)").cyan().bold().to_string())
        .allow_empty(true)
        .interact_text()?;

    let https_url = if https.is_empty() {
        None
    } else {
        Some(Url::parse(&https).map_err(|e| anyhow::anyhow!("invalid proxy https URL: {e}"))?)
    };

    let no_proxy: String = Input::new()
        .with_prompt(style("🚫 proxy no_proxy (blank to skip)").cyan().bold().to_string())
        .allow_empty(true)
        .interact_text()?;

    Ok(Some(Proxy {
        http: Some(http_url),
        https: https_url,
        no_proxy: if no_proxy.is_empty() { None } else { Some(no_proxy) },
    }))
}
