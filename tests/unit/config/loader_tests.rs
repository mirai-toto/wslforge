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
"#,
    )
    .expect("write config");

    let config = load_yaml(&path).expect("load yaml");
    assert_eq!(config.instances.len(), 1);
    let instance = config
        .instances
        .get("workstation")
        .expect("instance key should match hostname");
    assert_eq!(instance.username, "coder");
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
