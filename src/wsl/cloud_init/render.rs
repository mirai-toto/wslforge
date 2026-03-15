use crate::config::Profile;
use crate::wsl::helpers::crypto::hash_password_sha512;
use minijinja::Environment;
use serde::Serialize;

#[derive(Serialize)]
struct CloudInitRenderContext {
    profile: CloudInitRenderProfile,
    password_hash: Option<String>,
}

#[derive(Serialize)]
struct CloudInitRenderProfile {
    override_instance: bool,
    hostname: String,
    username: String,
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    no_proxy: Option<String>,
}

impl From<&Profile> for CloudInitRenderProfile {
    fn from(profile: &Profile) -> Self {
        Self {
            override_instance: profile.override_instance,
            hostname: profile.hostname.clone(),
            username: profile.username.clone(),
            http_proxy: profile.http_proxy.as_ref().map(ToString::to_string),
            https_proxy: profile.https_proxy.as_ref().map(ToString::to_string),
            no_proxy: profile.no_proxy.clone(),
        }
    }
}

pub fn render(raw: &str, profile: &Profile) -> anyhow::Result<String> {
    let mut env = Environment::new();
    env.add_template("cloud-init.user-data", raw)
        .map_err(|e| anyhow::anyhow!("cloud-init template parse error: {e}"))?;

    let template = env
        .get_template("cloud-init.user-data")
        .map_err(|e| anyhow::anyhow!("cloud-init template load error: {e}"))?;

    let password_hash = match profile.password.as_deref() {
        Some(password) => Some(hash_password_sha512(password)?),
        None => None,
    };

    let context = CloudInitRenderContext {
        profile: profile.into(),
        password_hash,
    };

    template
        .render(context)
        .map_err(|e| anyhow::anyhow!("cloud-init template render error: {e}"))
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/render_tests.rs"]
mod render_tests;
