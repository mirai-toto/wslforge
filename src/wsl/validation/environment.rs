use crate::wsl::engine::WslEngine;
use crate::wsl::helpers::command_error;
use encoding_rs::UTF_16LE;
use log::info;
use std::process::Command;

pub fn check_environment(engine: &dyn WslEngine) -> anyhow::Result<()> {
    validate_wsl_installed(engine)?;
    validate_windows_features(&["Microsoft-Windows-Subsystem-Linux", "VirtualMachinePlatform"])?;
    Ok(())
}

pub fn validate_wsl_installed(engine: &dyn WslEngine) -> anyhow::Result<()> {
    let output: std::process::Output = engine.status()?;
    if output.status.success() {
        info!("✅ WSL is installed");
        Ok(())
    } else {
        Err(command_error("WSL is not installed", &output))
    }
}

pub fn validate_windows_features(feature_names: &[&str]) -> anyhow::Result<()> {
    let mut disabled: Vec<&str> = Vec::new();
    for feature_name in feature_names {
        if is_windows_feature_enabled(feature_name)? {
            info!("✅ {feature_name} is enabled");
        } else {
            disabled.push(*feature_name);
        }
    }
    if !disabled.is_empty() {
        anyhow::bail!("required Windows feature(s) are disabled: {}", disabled.join(", "));
    }
    Ok(())
}

pub fn validate_wsl_distro_name(engine: &dyn WslEngine, name: &str) -> anyhow::Result<()> {
    if !is_valid_wsl_distro_name(engine, name)? {
        anyhow::bail!("unknown WSL distro name: {name}");
    }
    Ok(())
}

fn is_valid_wsl_distro_name(engine: &dyn WslEngine, name: &str) -> anyhow::Result<bool> {
    let output: std::process::Output = engine.list_online_distros()?;

    if !output.status.success() {
        anyhow::bail!("wsl.exe --list --online failed with status {}", output.status);
    }

    let (text, _, _): (std::borrow::Cow<'_, str>, &encoding_rs::Encoding, bool) = UTF_16LE.decode(&output.stdout);

    let ids: Vec<String> = text
        .lines()
        .map(str::trim)
        .skip_while(|l| !l.starts_with("NAME"))
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();

    Ok(ids.iter().any(|id| id.eq_ignore_ascii_case(name)))
}

fn is_windows_feature_enabled(feature_name: &str) -> anyhow::Result<bool> {
    let output: std::process::Output = Command::new("dism.exe")
        .args([
            "/English",
            "/online",
            "/Get-FeatureInfo",
            &format!("/featureName:{feature_name}"),
        ])
        .output()?;

    if !output.status.success() {
        return Err(command_error(
            &format!("dism.exe failed for feature '{feature_name}'"),
            &output,
        ));
    }

    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.trim() == "State : Enabled"))
}
