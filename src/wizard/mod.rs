mod file_path_completer;
mod navigation;
mod prompts;

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use console::style;
use inquire::{Password, PasswordDisplayMode, Select, Text};

use crate::config::{CloudInitSource, Config, ImageSource, Instance, Proxy, ScriptConfig};
use navigation::{is_anyhow_back, is_back, Step};
use prompts::{prompt_cloud_init, prompt_image, prompt_proxy};

pub const HELP: &str = "esc to go back | ctrl+c to abort";

/// Interactively builds a configuration for one WSL instance.
///
/// # Errors
///
/// Returns an error if a prompt fails or a prompted value cannot be parsed.
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

/// Prompts the user to confirm provisioning.
///
/// # Errors
///
/// Returns an error if the confirmation prompt fails.
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
    let mut answers = PromptAnswers::default();

    let image = loop {
        let going_forward = step != prev_step.prev() || step == Step::Hostname;
        match answers.prompt_step(step, going_forward)? {
            PromptAction::Advance => {}
            PromptAction::Back => {
                step = step.prev();
                prev_step = step;
                continue;
            }
            PromptAction::Retry => continue,
            PromptAction::Image(image) => break image,
        }

        prev_step = step;
        step = step.next();
    };

    Ok(answers.into_instance(image))
}

#[derive(Default)]
struct PromptAnswers {
    name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    override_instance: Option<bool>,
    proxy: ProxyAnswer,
    cloud_init: Option<(Option<CloudInitSource>, bool)>,
}

#[derive(Default)]
enum ProxyAnswer {
    #[default]
    Unanswered,
    NotConfigured,
    Configured(Proxy),
}

enum PromptAction {
    Advance,
    Back,
    Retry,
    Image(ImageSource),
}

impl PromptAnswers {
    fn prompt_step(&mut self, step: Step, going_forward: bool) -> anyhow::Result<PromptAction> {
        match step {
            Step::Hostname => self.prompt_hostname(going_forward),
            Step::Username => self.prompt_username(),
            Step::Password => self.prompt_password(),
            Step::Override => self.prompt_override(),
            Step::Proxy => self.prompt_proxy(going_forward),
            Step::CloudInit => self.prompt_cloud_init(going_forward),
            Step::Image => Self::prompt_image(going_forward),
        }
    }

    fn prompt_hostname(&mut self, going_forward: bool) -> anyhow::Result<PromptAction> {
        if going_forward {
            eprintln!(
                "{}",
                style("── Instance ─────────────────────────────────────────").dim()
            );
        }
        let default = self.name.as_deref().unwrap_or("Ubuntu");
        match Text::new("🏠  name")
            .with_default(default)
            .with_initial_value(default)
            .with_help_message(HELP)
            .prompt()
        {
            Ok(name) => {
                self.name = Some(name);
                Ok(PromptAction::Advance)
            }
            Err(error) if is_back(&error) => Ok(PromptAction::Advance),
            Err(error) => Err(error.into()),
        }
    }

    fn prompt_username(&mut self) -> anyhow::Result<PromptAction> {
        let prefill = self
            .username
            .clone()
            .unwrap_or_else(|| std::env::var("USERNAME").unwrap_or_default());
        match Text::new("👤  username (blank to skip)")
            .with_initial_value(&prefill)
            .with_help_message(HELP)
            .prompt()
        {
            Ok(username) => {
                self.username = Some(username);
                Ok(PromptAction::Advance)
            }
            Err(error) if is_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error.into()),
        }
    }

    fn prompt_password(&mut self) -> anyhow::Result<PromptAction> {
        match Password::new("🔑  password (blank to skip)")
            .with_display_toggle_enabled()
            .with_display_mode(PasswordDisplayMode::Masked)
            .with_help_message(HELP)
            .prompt()
        {
            Ok(password) => {
                self.password = Some(password);
                Ok(PromptAction::Advance)
            }
            Err(error) if is_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error.into()),
        }
    }

    fn prompt_override(&mut self) -> anyhow::Result<PromptAction> {
        let cursor = usize::from(self.override_instance == Some(true));
        match Select::new("♻️  override existing instance?", vec!["no", "yes"])
            .with_starting_cursor(cursor)
            .with_help_message(HELP)
            .prompt()
        {
            Ok(value) => {
                self.override_instance = Some(value == "yes");
                Ok(PromptAction::Advance)
            }
            Err(error) if is_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error.into()),
        }
    }

    fn prompt_proxy(&mut self, going_forward: bool) -> anyhow::Result<PromptAction> {
        if going_forward {
            eprintln!(
                "{}",
                style("── Proxy ────────────────────────────────────────────").dim()
            );
        }
        let cursor = usize::from(matches!(&self.proxy, ProxyAnswer::Configured(_)));
        match Select::new("🌐  configure proxy?", vec!["no", "yes"])
            .with_starting_cursor(cursor)
            .with_help_message(HELP)
            .prompt()
        {
            Ok("yes") => match prompt_proxy() {
                Ok(proxy) => {
                    self.proxy = proxy.map_or(ProxyAnswer::NotConfigured, ProxyAnswer::Configured);
                    Ok(PromptAction::Advance)
                }
                Err(error) if is_anyhow_back(&error) => Ok(PromptAction::Retry),
                Err(error) => Err(error),
            },
            Ok(_) => {
                self.proxy = ProxyAnswer::NotConfigured;
                Ok(PromptAction::Advance)
            }
            Err(error) if is_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error.into()),
        }
    }

    fn prompt_cloud_init(&mut self, going_forward: bool) -> anyhow::Result<PromptAction> {
        if going_forward {
            eprintln!(
                "{}",
                style("── Cloud-init ───────────────────────────────────────").dim()
            );
        }
        match prompt_cloud_init() {
            Ok(cloud_init) => {
                self.cloud_init = Some(cloud_init);
                Ok(PromptAction::Advance)
            }
            Err(error) if is_anyhow_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error),
        }
    }

    fn prompt_image(going_forward: bool) -> anyhow::Result<PromptAction> {
        if going_forward {
            eprintln!(
                "{}",
                style("── Image ────────────────────────────────────────────").dim()
            );
        }
        match prompt_image() {
            Ok(image) => Ok(PromptAction::Image(image)),
            Err(error) if is_anyhow_back(&error) => Ok(PromptAction::Back),
            Err(error) => Err(error),
        }
    }

    fn into_instance(self, image: ImageSource) -> (String, Instance) {
        let instance_name = self.name.unwrap();
        let (cloud_init, default_cloud_init) = self.cloud_init.unwrap();
        let username = self.username.unwrap();

        let instance = Instance {
            override_instance: self.override_instance.unwrap(),
            name: instance_name.clone(),
            user_home: match non_empty(username.clone()).as_deref() {
                Some("root") | None => "/root".to_string(),
                Some(user) => format!("/home/{user}"),
            },
            username: non_empty(username),
            password: non_empty(self.password.unwrap()),
            proxy: match self.proxy {
                ProxyAnswer::Unanswered | ProxyAnswer::NotConfigured => None,
                ProxyAnswer::Configured(proxy) => Some(proxy),
            },
            vars: HashMap::default(),
            files: vec![],
            scripts: ScriptConfig::default(),
            install_dir: PathBuf::from("%userprofile%/VMs"),
            default_cloud_init,
            cloud_init,
            cloud_init_timeout: None,
            image,
        };

        (instance_name, instance)
    }
}

fn non_empty(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
