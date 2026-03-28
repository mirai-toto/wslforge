use super::load_yaml;
use std::fs;

#[test]
fn loads_instances_map_format() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(
        &path,
        r#"
instances:
  dev:
    hostname: devbox
    username: coder
    install_dir: "%userprofile%/VMs"
    image:
      type: distro
      name: Ubuntu
"#,
    )
    .expect("write config");

    let config = load_yaml(&path).expect("load yaml");
    assert_eq!(config.instances.len(), 1);
    assert_eq!(config.instances["dev"].hostname, "devbox");
}

#[test]
fn loads_single_instance_and_uses_hostname_as_key() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(
        &path,
        r#"
hostname: workstation
username: coder
install_dir: "%userprofile%/VMs"
image:
  type: distro
  name: Ubuntu
"#,
    )
    .expect("write config");

    let config = load_yaml(&path).expect("load yaml");
    assert_eq!(config.instances.len(), 1);
    let instance = config
        .instances
        .get("workstation")
        .expect("instance key should match hostname");
    assert_eq!(instance.username.as_deref(), Some("coder"));
}

#[test]
fn rejects_config_missing_hostname() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "username: coder\nimage:\n  type: distro\n  name: Ubuntu\n").expect("write config");

    let err = load_yaml(&path).expect_err("missing hostname should fail");
    assert!(
        err.to_string().contains("hostname"),
        "error should mention hostname: {err}"
    );
}

#[test]
fn succeeds_with_missing_username() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "hostname: myhost\nimage:\n  type: distro\n  name: Ubuntu\n").expect("write config");

    let config = load_yaml(&path).expect("missing username should not fail");
    assert!(config.instances["myhost"].username.is_none());
}

#[test]
fn succeeds_with_missing_image_and_uses_default() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "hostname: myhost\nusername: coder\n").expect("write config");

    let config = load_yaml(&path).expect("missing image should not fail");
    assert!(
        matches!(config.instances["myhost"].image, crate::config::ImageSource::Distro { ref name } if name == "Ubuntu")
    );
}

#[test]
fn uses_default_install_dir_when_omitted() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "hostname: myhost\n").expect("write config");

    let config = load_yaml(&path).expect("load yaml");
    assert_eq!(
        config.instances["myhost"].install_dir,
        std::path::Path::new("%userprofile%/VMs")
    );
}

#[test]
fn reports_both_formats_for_invalid_yaml() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "instances: [").expect("write invalid config");

    let err = load_yaml(&path).expect_err("yaml should be invalid");
    let msg = err.to_string();
    assert!(msg.contains("instances format error"));
    assert!(msg.contains("single-instance format error"));
    assert!(msg.contains("Expected either:"));
}
