mod file_path_completer;
mod navigation;
mod prompts;

use std::collections::BTreeMap;
use std::path::PathBuf;

use console::style;
use inquire::{Password, PasswordDisplayMode, Select, Text};

use crate::config::{Config, Instance};
use navigation::{is_anyhow_back, is_back, Step};
use prompts::{prompt_cloud_init, prompt_image, prompt_proxy};

pub const HELP: &str = "esc to go back | ctrl+c to abort";

pub fn run() -> anyhow::Result<Config> {
    eprintln!(
        "{}",
        style("⚒️  No config file found. Let's configure your WSL instance.").bold()
    );
    eprintln!();

    let (name, instance) = prompt_instance()?;

    eprintln!();

    Ok(Config {
        instances: BTreeMap::from([(name, instance)]),
    })
}

pub fn confirm_provision() -> anyhow::Result<()> {
    let choice = Select::new("🚀  ready to provision?", vec!["yes, proceed", "no, abort"])
        .with_help_message(HELP)
        .prompt()?;

    if choice != "yes, proceed" {
        eprintln!("{}", style("Aborted.").yellow());
        std::process::exit(0);
    }

    Ok(())
}

fn prompt_instance() -> anyhow::Result<(String, Instance)> {
    let mut step = Step::Hostname;
    let mut prev_step = Step::Hostname;

    let mut name: Option<String> = None;
    let mut username_raw: Option<String> = None;
    let mut password_raw: Option<String> = None;
    let mut override_instance: Option<bool> = None;
    let mut proxy = None;
    let mut cloud_init_raw: Option<(Option<crate::config::CloudInitSource>, bool)> = None;

    let image = loop {
        let going_forward = step != prev_step.prev() || step == Step::Hostname;

        match step {
            Step::Hostname => {
                if going_forward {
                    eprintln!(
                        "{}",
                        style("── Instance ─────────────────────────────────────────").dim()
                    );
                }
                let default = name.as_deref().unwrap_or("Ubuntu");
                match Text::new("🏠  name")
                    .with_default(default)
                    .with_initial_value(default)
                    .with_help_message(HELP)
                    .prompt()
                {
                    Ok(v) => {
                        name = Some(v);
                    }
                    Err(e) if is_back(&e) => {}
                    Err(e) => return Err(e.into()),
                }
            }
            Step::Username => {
                let prefill = username_raw
                    .clone()
                    .unwrap_or_else(|| std::env::var("USERNAME").unwrap_or_default());
                match Text::new("👤  username (blank to skip)")
                    .with_initial_value(&prefill)
                    .with_help_message(HELP)
                    .prompt()
                {
                    Ok(v) => {
                        username_raw = Some(v);
                    }
                    Err(e) if is_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Step::Password => {
                match Password::new("🔑  password (blank to skip)")
                    .with_display_toggle_enabled()
                    .with_display_mode(PasswordDisplayMode::Masked)
                    .with_help_message(HELP)
                    .prompt()
                {
                    Ok(v) => {
                        password_raw = Some(v);
                    }
                    Err(e) if is_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Step::Override => {
                let cursor = if override_instance == Some(true) { 1 } else { 0 };
                match Select::new("♻️  override existing instance?", vec!["no", "yes"])
                    .with_starting_cursor(cursor)
                    .with_help_message(HELP)
                    .prompt()
                {
                    Ok(v) => {
                        override_instance = Some(v == "yes");
                    }
                    Err(e) if is_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Step::Proxy => {
                if going_forward {
                    eprintln!(
                        "{}",
                        style("── Proxy ────────────────────────────────────────────").dim()
                    );
                }
                let cursor = if matches!(proxy, Some(Some(_))) { 1 } else { 0 };
                match Select::new("🌐  configure proxy?", vec!["no", "yes"])
                    .with_starting_cursor(cursor)
                    .with_help_message(HELP)
                    .prompt()
                {
                    Ok("yes") => match prompt_proxy() {
                        Ok(p) => {
                            proxy = Some(p);
                        }
                        Err(e) if is_anyhow_back(&e) => continue,
                        Err(e) => return Err(e),
                    },
                    Ok(_) => {
                        proxy = Some(None);
                    }
                    Err(e) if is_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e.into()),
                }
            }
            Step::CloudInit => {
                if going_forward {
                    eprintln!(
                        "{}",
                        style("── Cloud-init ───────────────────────────────────────").dim()
                    );
                }
                match prompt_cloud_init() {
                    Ok(v) => {
                        cloud_init_raw = Some(v);
                    }
                    Err(e) if is_anyhow_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
            Step::Image => {
                if going_forward {
                    eprintln!(
                        "{}",
                        style("── Image ────────────────────────────────────────────").dim()
                    );
                }
                match prompt_image() {
                    Ok(v) => break v,
                    Err(e) if is_anyhow_back(&e) => {
                        step = step.prev();
                        prev_step = step;
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        prev_step = step;
        step = step.next();
    };

    let name = name.unwrap();
    let (cloud_init, default_cloud_init) = cloud_init_raw.unwrap();

    let instance = Instance {
        override_instance: override_instance.unwrap(),
        name: name.clone(),
        user_home: match non_empty(username_raw.clone().unwrap()).as_deref() {
            Some("root") | None => "/root".to_string(),
            Some(u) => format!("/home/{u}"),
        },
        username: non_empty(username_raw.unwrap()),
        password: non_empty(password_raw.unwrap()),
        proxy: proxy.unwrap(),
        vars: Default::default(),
        files: vec![],
        scripts: Default::default(),
        install_dir: PathBuf::from("%userprofile%/VMs"),
        default_cloud_init,
        cloud_init,
        image,
    };

    Ok((name, instance))
}

fn non_empty(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
