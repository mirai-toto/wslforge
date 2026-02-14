pub(crate) fn resolve_userprofile_dir() -> anyhow::Result<std::path::PathBuf> {
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Ok(std::path::PathBuf::from(path));
    }
    anyhow::bail!("USERPROFILE is not set; cannot place cloud-init user-data")
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/helpers/userprofile_tests.rs"]
mod userprofile_tests;
