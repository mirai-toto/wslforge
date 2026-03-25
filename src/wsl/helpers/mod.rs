use crate::config::SourcePath;
use sha_crypt::{sha512_simple, Sha512Params, ROUNDS_DEFAULT};
use std::path::{Path, PathBuf};

pub(crate) fn hash_password_sha512(password: &str) -> anyhow::Result<String> {
    let params: Sha512Params =
        Sha512Params::new(ROUNDS_DEFAULT).map_err(|e| anyhow::anyhow!("invalid sha512-crypt params: {e:?}"))?;
    sha512_simple(password, &params).map_err(|e| anyhow::anyhow!("password hashing failed: {e:?}"))
}

// Expands env vars, supporting both %VAR% and $VAR styles.
pub(crate) fn expand_env_vars(raw: &str) -> anyhow::Result<String> {
    let percent_expanded: String = expand_str::expand_string_with_env(raw)
        .map_err(|e| anyhow::anyhow!("environment variable expansion failed: {e}"))?;
    let expanded: std::borrow::Cow<'_, str> = shellexpand::env(&percent_expanded)
        .map_err(|e| anyhow::anyhow!("environment variable '{}' is not set (from '{}')", e.var_name, raw))?;
    Ok(expanded.into_owned())
}

pub(crate) fn expand_path(raw: &Path) -> anyhow::Result<PathBuf> {
    let expanded: String = expand_env_vars(&raw.to_string_lossy())?;
    Ok(PathBuf::from(expanded))
}

pub(crate) fn resolve_install_dir(install_dir: &Path, hostname: &str) -> anyhow::Result<PathBuf> {
    let expanded: PathBuf = expand_path(install_dir)?;
    Ok(expanded.join(hostname))
}

pub(crate) fn resolve_userprofile_dir() -> anyhow::Result<PathBuf> {
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(path));
    }
    anyhow::bail!("USERPROFILE is not set; cannot place cloud-init user-data")
}

pub(crate) enum ResolvedPath {
    Local(PathBuf),
    Temp(tempfile::TempPath),
}

impl ResolvedPath {
    pub(crate) fn as_path(&self) -> &Path {
        match self {
            ResolvedPath::Local(p) => p.as_path(),
            ResolvedPath::Temp(t) => t.as_ref(),
        }
    }
}

pub(crate) fn resolve_source_path(path: &SourcePath) -> anyhow::Result<ResolvedPath> {
    match path {
        SourcePath::Local(p) => Ok(ResolvedPath::Local(expand_path(p)?)),
        SourcePath::Remote(url) => {
            let response = ureq::get(url.as_str())
                .call()
                .map_err(|e| anyhow::anyhow!("failed to download '{}': {e}", url))?;
            let mut tmp = tempfile::NamedTempFile::new()?;
            std::io::copy(&mut response.into_reader(), tmp.as_file_mut())?;
            // Close the file handle so wsl.exe --import can open the file on Windows.
            Ok(ResolvedPath::Temp(tmp.into_temp_path()))
        }
    }
}

pub(crate) fn command_error(description: &str, output: &std::process::Output) -> anyhow::Error {
    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let stderr: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
    anyhow::anyhow!(
        "{description} with status {}\n{}\n{}",
        output.status,
        stdout.trim(),
        stderr.trim()
    )
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/helpers/path_tests.rs"]
mod path_tests;

#[cfg(test)]
#[path = "../../../tests/unit/wsl/helpers/userprofile_tests.rs"]
mod userprofile_tests;
