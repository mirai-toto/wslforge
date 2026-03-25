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
    let env: Environment<'_> = Environment::new();

    let password_hash = instance.password.as_deref().map(hash_password_sha512).transpose()?;

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

    env.render_str(raw, context)
        .map_err(|e| anyhow::anyhow!("cloud-init template error: {e}"))
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/render_tests.rs"]
mod render_tests;
