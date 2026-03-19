use crate::config::Instance;
use crate::wsl::helpers::hash_password_sha512;
use minijinja::Environment;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct CloudInitRenderProxy {
    http: Option<String>,
    https: Option<String>,
    no_proxy: Option<String>,
}

#[derive(Serialize)]
struct CloudInitRenderContext {
    hostname: String,
    username: String,
    password_hash: Option<String>,
    proxy: Option<CloudInitRenderProxy>,
    vars: HashMap<String, String>,
}

pub fn render(raw: &str, instance: &Instance) -> anyhow::Result<String> {
    let mut env: Environment<'_> = Environment::new();
    env.add_template("cloud-init.user-data", raw)
        .map_err(|e| anyhow::anyhow!("cloud-init template parse error: {e}"))?;

    let template: minijinja::Template<'_, '_> = env
        .get_template("cloud-init.user-data")
        .map_err(|e| anyhow::anyhow!("cloud-init template load error: {e}"))?;

    let password_hash: Option<String> = match instance.password.as_deref() {
        Some(password) => Some(hash_password_sha512(password)?),
        None => None,
    };

    let context: CloudInitRenderContext = CloudInitRenderContext {
        hostname: instance.hostname.clone(),
        username: instance.username.clone(),
        password_hash,
        proxy: instance.proxy.as_ref().map(|p| CloudInitRenderProxy {
            http: p.http.as_ref().map(ToString::to_string),
            https: p.https.as_ref().map(ToString::to_string),
            no_proxy: p.no_proxy.clone(),
        }),
        vars: instance.vars.clone(),
    };

    template
        .render(context)
        .map_err(|e| anyhow::anyhow!("cloud-init template render error: {e}"))
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/render_tests.rs"]
mod render_tests;
