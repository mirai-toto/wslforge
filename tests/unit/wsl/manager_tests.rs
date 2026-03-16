use super::WslManager;
use crate::config::Profile;
use crate::wsl::engine::WslEngine;
use crate::wsl::{Outcome, ProvisionEvent, RunOptions};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

struct FakeEngine {
    instance_exists: bool,
    fail_create_from_file: bool,
    calls: Arc<Mutex<Vec<String>>>,
}

impl FakeEngine {
    fn new(instance_exists: bool, fail_create_from_file: bool) -> (Self, Arc<Mutex<Vec<String>>>) {
        let calls = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                instance_exists,
                fail_create_from_file,
                calls: calls.clone(),
            },
            calls,
        )
    }
}

impl WslEngine for FakeEngine {
    fn status(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("not used in manager create_instance tests")
    }

    fn update(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("not used in manager create_instance tests")
    }

    fn list_online_distros(&self) -> anyhow::Result<std::process::Output> {
        anyhow::bail!("not used in manager create_instance tests")
    }

    fn instance_exists(&self, name: &str) -> anyhow::Result<bool> {
        self.calls
            .lock()
            .expect("lock calls")
            .push(format!("instance_exists:{name}"));
        Ok(self.instance_exists)
    }

    fn delete_instance(&self, name: &str) -> anyhow::Result<()> {
        self.calls
            .lock()
            .expect("lock calls")
            .push(format!("delete_instance:{name}"));
        Ok(())
    }

    fn create_from_file(&self, name: &str, _install_dir: &Path, _rootfs_tar: &Path) -> anyhow::Result<()> {
        self.calls
            .lock()
            .expect("lock calls")
            .push(format!("create_from_file:{name}"));
        if self.fail_create_from_file {
            anyhow::bail!("create from file failed");
        }
        Ok(())
    }

    fn create_from_distro(&self, distro_name: &str, name: &str) -> anyhow::Result<()> {
        self.calls
            .lock()
            .expect("lock calls")
            .push(format!("create_from_distro:{distro_name}:{name}"));
        Ok(())
    }
}

fn file_image_profile(image_path: &Path) -> Profile {
    serde_yaml::from_str(&format!(
        r#"
hostname: devbox
username: devuser
install_dir: /tmp/wslforge-install
image:
  type: file
  path: {}
"#,
        image_path.display()
    ))
    .expect("deserialize profile")
}

fn file_image_profile_with_override(image_path: &Path, override_instance: bool) -> Profile {
    serde_yaml::from_str(&format!(
        r#"
hostname: devbox
username: devuser
override: {}
install_dir: /tmp/wslforge-install
image:
  type: file
  path: {}
"#,
        override_instance,
        image_path.display()
    ))
    .expect("deserialize profile")
}

fn create_temp_tar_file(dir: &tempfile::TempDir) -> PathBuf {
    let path = dir.path().join("rootfs.tar");
    std::fs::write(&path, b"fake rootfs").expect("write fake tar");
    path
}

#[test]
fn create_instance_returns_already_exists_when_present_without_override() {
    let profile: Profile = serde_yaml::from_str(
        r#"
hostname: devbox
username: devuser
override: false
"#,
    )
    .expect("deserialize profile");
    let (engine, calls) = FakeEngine::new(true, false);
    let manager = WslManager::new(Box::new(engine));

    let report = manager
        .create_instance(&profile, RunOptions::default())
        .expect("create instance should return report");

    assert_eq!(report.outcome, Outcome::AlreadyExists);
    assert_eq!(
        report.events,
        vec![ProvisionEvent::InstanceCheckStarted, ProvisionEvent::InstanceExists]
    );
    assert_eq!(calls.lock().expect("lock calls").as_slice(), ["instance_exists:devbox"]);
}

#[test]
fn create_instance_dry_run_skips_engine_create_after_prepare() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let image_path = create_temp_tar_file(&dir);
    let profile = file_image_profile(&image_path);
    let (engine, calls) = FakeEngine::new(false, false);
    let manager = WslManager::new(Box::new(engine));

    let report = manager
        .create_instance(
            &profile,
            RunOptions {
                dry_run: true,
                debug: false,
            },
        )
        .expect("dry run should succeed");

    assert_eq!(report.outcome, Outcome::Skipped);
    assert_eq!(
        report.events,
        vec![
            ProvisionEvent::InstanceCheckStarted,
            ProvisionEvent::InstanceMissing,
            ProvisionEvent::CloudInitNotConfigured,
            ProvisionEvent::CreateDryRun,
        ]
    );
    assert_eq!(calls.lock().expect("lock calls").as_slice(), ["instance_exists:devbox"]);
}

#[test]
fn create_instance_returns_error_when_engine_create_fails() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let image_path = create_temp_tar_file(&dir);
    let profile = file_image_profile(&image_path);
    let (engine, calls) = FakeEngine::new(false, true);
    let manager = WslManager::new(Box::new(engine));

    let err = manager
        .create_instance(&profile, RunOptions::default())
        .expect_err("engine create failure should bubble up");
    assert!(err.to_string().contains("create from file failed"));
    assert_eq!(
        calls.lock().expect("lock calls").as_slice(),
        ["instance_exists:devbox", "create_from_file:devbox"]
    );
}

#[test]
fn create_instance_override_dry_run_reports_delete_dry_run_and_skips_create() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let image_path = create_temp_tar_file(&dir);
    let profile = file_image_profile_with_override(&image_path, true);
    let (engine, calls) = FakeEngine::new(true, false);
    let manager = WslManager::new(Box::new(engine));

    let report = manager
        .create_instance(
            &profile,
            RunOptions {
                dry_run: true,
                debug: false,
            },
        )
        .expect("dry run should succeed");

    assert_eq!(report.outcome, Outcome::Skipped);
    assert_eq!(
        report.events,
        vec![
            ProvisionEvent::InstanceCheckStarted,
            ProvisionEvent::InstanceExists,
            ProvisionEvent::OverrideRequested,
            ProvisionEvent::CloudInitNotConfigured,
            ProvisionEvent::OverrideExistingInstance,
            ProvisionEvent::DeleteDryRun,
            ProvisionEvent::CreateDryRun,
        ]
    );
    assert_eq!(calls.lock().expect("lock calls").as_slice(), ["instance_exists:devbox"]);
}

#[test]
fn create_instance_override_existing_deletes_then_creates() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let image_path = create_temp_tar_file(&dir);
    let profile = file_image_profile_with_override(&image_path, true);
    let (engine, calls) = FakeEngine::new(true, false);
    let manager = WslManager::new(Box::new(engine));

    let report = manager
        .create_instance(&profile, RunOptions::default())
        .expect("override create should succeed");

    assert_eq!(report.outcome, Outcome::Created);
    assert_eq!(
        report.events,
        vec![
            ProvisionEvent::InstanceCheckStarted,
            ProvisionEvent::InstanceExists,
            ProvisionEvent::OverrideRequested,
            ProvisionEvent::CloudInitNotConfigured,
            ProvisionEvent::OverrideExistingInstance,
            ProvisionEvent::DeleteStarted,
            ProvisionEvent::DeleteCompleted,
            ProvisionEvent::CreateStarted,
        ]
    );
    assert_eq!(
        calls.lock().expect("lock calls").as_slice(),
        [
            "instance_exists:devbox",
            "delete_instance:devbox",
            "create_from_file:devbox"
        ]
    );
}
