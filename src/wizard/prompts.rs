use std::path::PathBuf;

use inquire::validator::Validation;
use inquire::{Select, Text};
use url::Url;

use crate::config::{CloudInitSource, ImageSource, Proxy};
use crate::wizard::file_path_completer::FilePathCompleter;
use crate::wizard::HELP;

pub fn prompt_cloud_init() -> anyhow::Result<(Option<CloudInitSource>, bool)> {
    let choice = Select::new("☁️  cloud-init", vec!["default (auto-generated)", "file", "none"])
        .with_help_message(HELP)
        .prompt()?;

    match choice {
        "file" => {
            let path = Text::new("📄  cloud-init file path")
                .with_autocomplete(FilePathCompleter::default())
                .with_help_message(HELP)
                .with_validator(|input: &str| {
                    let p = std::path::Path::new(input);
                    if p.is_dir() {
                        Ok(Validation::Invalid("please select a file, not a directory".into()))
                    } else if !p.exists() {
                        Ok(Validation::Invalid("file does not exist".into()))
                    } else {
                        Ok(Validation::Valid)
                    }
                })
                .prompt()?;
            Ok((
                Some(CloudInitSource::File {
                    path: PathBuf::from(path),
                }),
                false,
            ))
        }
        "none" => Ok((None, false)),
        _ => Ok((None, true)),
    }
}

pub fn prompt_image() -> anyhow::Result<ImageSource> {
    let choice = Select::new(
        "🐧  image source",
        vec!["distro name (e.g. Ubuntu, Debian)", "rootfs file or URL"],
    )
    .with_help_message(HELP)
    .prompt()?;

    if choice == "distro name (e.g. Ubuntu, Debian)" {
        let name = Text::new("🐧  distro name ")
            .with_default("Ubuntu")
            .with_help_message(HELP)
            .prompt()?;
        Ok(ImageSource::Distro { name })
    } else {
        let path = Text::new("🗂️  rootfs path or URL").with_help_message(HELP).prompt()?;
        Ok(ImageSource::File {
            path: path.parse().unwrap(),
        })
    }
}

pub fn prompt_proxy() -> anyhow::Result<Option<Proxy>> {
    let http = Text::new("🌐  proxy http (blank to skip)")
        .with_help_message(HELP)
        .prompt()?;

    if http.is_empty() {
        return Ok(None);
    }

    let http_url = Url::parse(&http).map_err(|e| anyhow::anyhow!("invalid proxy http URL: {e}"))?;

    let https = Text::new("🔒  proxy https (blank to skip)")
        .with_help_message(HELP)
        .prompt()?;
    let https_url = non_empty(https)
        .map(|s| Url::parse(&s).map_err(|e| anyhow::anyhow!("invalid proxy https URL: {e}")))
        .transpose()?;

    let no_proxy = Text::new("🚫  no_proxy (blank to skip)")
        .with_help_message(HELP)
        .prompt()?;

    Ok(Some(Proxy {
        http: Some(http_url),
        https: https_url,
        no_proxy: non_empty(no_proxy),
    }))
}

fn non_empty(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
