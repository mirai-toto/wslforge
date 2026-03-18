use crate::config::Instance;
use crate::wsl::helpers::hash_password_sha512;
use minijinja::Environment;
use serde::Serialize;

#[derive(Serialize)]
struct CloudInitRenderContext {
    instance: CloudInitRenderInstance,
    password_hash: Option<String>,
}

#[derive(Serialize)]
struct CloudInitRenderInstance {
    override_instance: bool,
    hostname: String,
    username: String,
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    no_proxy: Option<String>,
}

impl From<&Instance> for CloudInitRenderInstance {
    fn from(instance: &Instance) -> Self {
        Self {
            override_instance: instance.override_instance,
            hostname: instance.hostname.clone(),
            username: instance.username.clone(),
            http_proxy: instance.http_proxy.as_ref().map(ToString::to_string),
            https_proxy: instance.https_proxy.as_ref().map(ToString::to_string),
            no_proxy: instance.no_proxy.clone(),
        }
    }
}

pub fn render(raw: &str, instance: &Instance) -> anyhow::Result<String> {
    let mut env = Environment::new();
    env.add_template("cloud-init.user-data", raw)
        .map_err(|e| anyhow::anyhow!("cloud-init template parse error: {e}"))?;

    let template = env
        .get_template("cloud-init.user-data")
        .map_err(|e| anyhow::anyhow!("cloud-init template load error: {e}"))?;

    let password_hash = match instance.password.as_deref() {
        Some(password) => Some(hash_password_sha512(password)?),
        None => None,
    };

    let context = CloudInitRenderContext {
        instance: instance.into(),
        password_hash,
    };

    template
        .render(context)
        .map_err(|e| anyhow::anyhow!("cloud-init template render error: {e}"))
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/render_tests.rs"]
mod render_tests;
