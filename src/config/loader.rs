use crate::config::{Config, Instance};
use anyhow::Context;
use std::collections::BTreeMap;
use std::{fs, path::Path};

fn format_yaml_error(path: &Path, err: &serde_yaml::Error) -> String {
    if let Some(loc) = err.location() {
        format!("{}:{}:{}: {}", path.display(), loc.line(), loc.column(), err)
    } else {
        format!("{}: {}", path.display(), err)
    }
}

pub fn load_yaml(path: &Path) -> anyhow::Result<Config> {
    let raw = fs::read_to_string(path).with_context(|| format!("unable to read config file: {}", path.display()))?;

    serde_yaml::from_str::<Config>(&raw).or_else(|root_err| {
        serde_yaml::from_str::<Instance>(&raw)
            .map(|instance| Config {
                instances: BTreeMap::from([(instance.hostname.clone(), instance)]),
            })
            .map_err(|instance_err| anyhow::anyhow!(
                "invalid yaml\n- instances format error: {}\n- single-instance format error: {}\n\nExpected either:\n- instances:\n    <name>:\n      <instance>\n- or a single instance object at the root",
                format_yaml_error(path, &root_err),
                format_yaml_error(path, &instance_err)
            ))
    })
}

#[cfg(test)]
#[path = "../../tests/unit/config/loader_tests.rs"]
mod loader_tests;
