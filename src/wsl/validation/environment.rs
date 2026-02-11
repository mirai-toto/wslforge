use crate::wsl::{EnvironmentEvent, EnvironmentReport};
use encoding_rs::UTF_16LE;
use log::debug;
use std::process::Command;

pub fn validate_environment(dry_run: bool) -> anyhow::Result<EnvironmentReport> {
    let mut events = check_environment()?;
    events.push(prepare_environment(dry_run)?);
    Ok(EnvironmentReport { events })
}

pub fn check_environment() -> anyhow::Result<Vec<EnvironmentEvent>> {
    let mut events = vec![validate_wsl_installed()?];
    events.extend(validate_windows_features(&[
        "Microsoft-Windows-Subsystem-Linux",
        "VirtualMachinePlatform",
    ])?);
    Ok(events)
}

pub fn prepare_environment(dry_run: bool) -> anyhow::Result<EnvironmentEvent> {
    update_wsl_version(dry_run)
}

pub fn validate_wsl_installed() -> anyhow::Result<EnvironmentEvent> {
    let output = Command::new("wsl.exe").arg("--status").output()?;
    if output.status.success() {
        Ok(EnvironmentEvent::WslInstalled)
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("⛔ WSL is not installed.\n{}\n{}", stdout.trim(), stderr.trim())
    }
}

pub fn update_wsl_version(dry_run: bool) -> anyhow::Result<EnvironmentEvent> {
    if dry_run {
        return Ok(EnvironmentEvent::WslUpdateDryRun);
    }
    let output = Command::new("wsl.exe").arg("--update").output()?;
    if output.status.success() {
        Ok(EnvironmentEvent::WslUpdateCompleted)
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("⛔ Failed to update WSL.\n{}\n{}", stdout.trim(), stderr.trim())
    }
}

pub fn validate_windows_features(feature_names: &[&str]) -> anyhow::Result<Vec<EnvironmentEvent>> {
    let mut disabled = Vec::new();
    let mut events = Vec::new();
    for feature_name in feature_names {
        match is_windows_feature_enabled(feature_name)? {
            true => events.push(EnvironmentEvent::WindowsFeatureEnabled((*feature_name).to_string())),
            false => disabled.push(*feature_name),
        }
    }
    if !disabled.is_empty() {
        anyhow::bail!("required Windows feature(s) are disabled: {}", disabled.join(", "));
    }
    Ok(events)
}

pub fn validate_wsl_distro_name(name: &str) -> anyhow::Result<()> {
    if !is_valid_wsl_distro_name(name)? {
        anyhow::bail!("unknown WSL distro name: {name}");
    }
    Ok(())
}

//
// OS interaction helpers
//

fn is_valid_wsl_distro_name(name: &str) -> anyhow::Result<bool> {
    let output = Command::new("wsl.exe").args(["--list", "--online"]).output()?;

    if !output.status.success() {
        anyhow::bail!("wsl.exe --list --online failed with status {}", output.status);
    }

    let (text, _, _) = UTF_16LE.decode(&output.stdout);

    let ids: Vec<String> = text
        .lines()
        .map(str::trim)
        .skip_while(|l| !l.starts_with("NAME"))
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(|l| l.split_whitespace().next().map(str::to_string))
        .collect();

    debug!("Available WSL online distros: {:?}", ids);
    Ok(ids.iter().any(|id| id.eq_ignore_ascii_case(name)))
}

fn is_windows_feature_enabled(feature_name: &str) -> anyhow::Result<bool> {
    let output = Command::new("dism.exe")
        .args([
            "/English",
            "/online",
            "/Get-FeatureInfo",
            &format!("/featureName:{feature_name}"),
        ])
        .output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "dism.exe failed for feature '{feature_name}' with status {}\n{}",
            output.status,
            stdout.trim(),
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line.trim() == "State : Enabled"))
}
