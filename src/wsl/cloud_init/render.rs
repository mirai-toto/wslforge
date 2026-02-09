use crate::config::Profile;
use crate::wsl::helpers::crypto::hash_password_sha512;
use minijinja::Environment;

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

    template
        .render(minijinja::context! { profile => profile, password_hash => password_hash })
        .map_err(|e| anyhow::anyhow!("cloud-init template render error: {e}"))
}
