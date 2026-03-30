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
    assert_eq!(config.instances["dev"].name, "dev");
}

#[test]
fn succeeds_with_missing_username() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(
        &path,
        "instances:\n  myhost:\n    image:\n      type: distro\n      name: Ubuntu\n",
    )
    .expect("write config");

    let config = load_yaml(&path).expect("missing username should not fail");
    assert!(config.instances["myhost"].username.is_none());
}

#[test]
fn succeeds_with_missing_image_and_uses_default() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "instances:\n  myhost:\n    username: coder\n").expect("write config");

    let config = load_yaml(&path).expect("missing image should not fail");
    assert!(
        matches!(config.instances["myhost"].image, crate::config::ImageSource::Distro { ref name } if name == "Ubuntu")
    );
}

#[test]
fn uses_default_install_dir_when_omitted() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "instances:\n  myhost:\n    username: coder\n").expect("write config");

    let config = load_yaml(&path).expect("load yaml");
    assert_eq!(
        config.instances["myhost"].install_dir,
        std::path::Path::new("%userprofile%/VMs")
    );
}

#[test]
fn reports_error_for_invalid_yaml() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let path = dir.path().join("config.yaml");
    fs::write(&path, "instances: [").expect("write invalid config");

    let err = load_yaml(&path).expect_err("yaml should be invalid");
    assert!(!err.to_string().is_empty());
}
