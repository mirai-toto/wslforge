use sha_crypt::{sha512_simple, Sha512Params, ROUNDS_DEFAULT};

// Expands env vars, supporting both %VAR% and $VAR styles.
pub(crate) fn expand_env_vars(raw: &str) -> anyhow::Result<String> {
    let percent_expanded = expand_str::expand_string_with_env(raw)
        .map_err(|e| anyhow::anyhow!("environment variable expansion failed: {e}"))?;
    let expanded = shellexpand::env(&percent_expanded).map_err(|e| {
        anyhow::anyhow!(
            "environment variable '{}' is not set (from '{}')",
            e.var_name,
            raw
        )
    })?;
    Ok(expanded.into_owned())
}

pub(crate) fn resolve_userprofile_dir() -> anyhow::Result<std::path::PathBuf> {
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Ok(std::path::PathBuf::from(path));
    }
    anyhow::bail!("USERPROFILE is not set; cannot place cloud-init user-data")
}

pub(crate) fn hash_password_sha512(password: &str) -> anyhow::Result<String> {
    let params = Sha512Params::new(ROUNDS_DEFAULT)
        .map_err(|e| anyhow::anyhow!("invalid sha512-crypt params: {e:?}"))?;
    sha512_simple(password, &params).map_err(|e| anyhow::anyhow!("password hashing failed: {e:?}"))
}
