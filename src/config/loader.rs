use crate::config::Config;
use anyhow::Context;
use std::{fs, path::Path};

fn format_yaml_error(path: &Path, err: &serde_yaml::Error) -> String {
    if let Some(loc) = err.location() {
        format!("{}:{}:{}: {}", path.display(), loc.line(), loc.column(), err)
    } else {
        format!("{}: {}", path.display(), err)
    }
}

pub fn load_yaml(path: &Path) -> anyhow::Result<Config> {
    let raw: String =
        fs::read_to_string(path).with_context(|| format!("unable to read config file: {}", path.display()))?;

    serde_yaml::from_str::<Config>(&raw)
        .map_err(|err| anyhow::anyhow!("{}", format_yaml_error(path, &err)))
        .map(|mut config| {
            for (key, instance) in &mut config.instances {
                instance.name = key.clone();
                instance.user_home = match instance.username.as_deref() {
                    Some("root") | None => "/root".to_string(),
                    Some(u) => format!("/home/{u}"),
                };
            }
            config
        })
}

#[cfg(test)]
#[path = "../../tests/unit/config/loader_tests.rs"]
mod loader_tests;
