use std::collections::BTreeMap;
use std::path::PathBuf;

use dialoguer::{Confirm, Input, Password};
use url::Url;

use crate::config::{CloudInitSource, Config, ImageSource, Instance, Proxy};

pub fn run() -> anyhow::Result<Config> {
    eprintln!("No config file found. Let's configure your WSL instance.\n");

    let hostname: String = Input::new()
        .with_prompt("hostname")
        .default("UbuntuWSL".into())
        .interact_text()?;

    let username: String = Input::new()
        .with_prompt("username")
        .default("wsluser".into())
        .interact_text()?;

    let password: String = Password::new()
        .with_prompt("password (blank to skip)")
        .allow_empty_password(true)
        .interact()?;

    let override_instance: bool = Confirm::new()
        .with_prompt("override existing instance?")
        .default(false)
        .interact()?;

    let proxy = if Confirm::new()
        .with_prompt("configure proxy?")
        .default(false)
        .interact()?
    {
        prompt_proxy()?
    } else {
        None
    };

    let cloud_init_path: String = Input::new()
        .with_prompt("cloud-init file path (blank to skip)")
        .allow_empty(true)
        .interact_text()?;

    let cloud_init = if cloud_init_path.is_empty() {
        None
    } else {
        Some(CloudInitSource::File { path: PathBuf::from(cloud_init_path) })
    };

    let image = prompt_image()?;

    let instance = Instance {
        override_instance,
        hostname: hostname.clone(),
        username,
        password: if password.is_empty() { None } else { Some(password) },
        proxy,
        vars: Default::default(),
        files: vec![],
        install_dir: PathBuf::from("%userprofile%/VMs"),
        cloud_init,
        image,
    };

    Ok(Config {
        instances: BTreeMap::from([(hostname, instance)]),
    })
}

fn prompt_image() -> anyhow::Result<ImageSource> {
    let use_file: bool = Confirm::new()
        .with_prompt("use a local rootfs file instead of a distro?")
        .default(false)
        .interact()?;

    if use_file {
        let path: String = Input::new()
            .with_prompt("rootfs file path")
            .interact_text()?;
        Ok(ImageSource::File { path: PathBuf::from(path) })
    } else {
        let name: String = Input::new()
            .with_prompt("distro name")
            .default("Ubuntu".into())
            .interact_text()?;
        Ok(ImageSource::Distro { name })
    }
}

fn prompt_proxy() -> anyhow::Result<Option<Proxy>> {
    let http: String = Input::new()
        .with_prompt("proxy http (blank to skip)")
        .allow_empty(true)
        .interact_text()?;

    if http.is_empty() {
        return Ok(None);
    }

    let http_url = Url::parse(&http)
        .map_err(|e| anyhow::anyhow!("invalid proxy http URL: {e}"))?;

    let https: String = Input::new()
        .with_prompt("proxy https (blank to skip)")
        .allow_empty(true)
        .interact_text()?;

    let https_url = if https.is_empty() {
        None
    } else {
        Some(Url::parse(&https).map_err(|e| anyhow::anyhow!("invalid proxy https URL: {e}"))?)
    };

    let no_proxy: String = Input::new()
        .with_prompt("proxy no_proxy (blank to skip)")
        .allow_empty(true)
        .interact_text()?;

    Ok(Some(Proxy {
        http: Some(http_url),
        https: https_url,
        no_proxy: if no_proxy.is_empty() { None } else { Some(no_proxy) },
    }))
}