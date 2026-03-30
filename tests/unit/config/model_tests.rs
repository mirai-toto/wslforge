use super::{CloudInitSource, ImageSource, Instance};
use std::path::Path;

#[test]
fn instance_deserialization_name_is_empty_by_default() {
    // name is #[serde(skip)] — always populated by the loader from the map key, never from YAML
    let instance: Instance = serde_yaml::from_str("{}\n").expect("bare instance should deserialize");
    assert!(instance.name.is_empty());
    assert!(instance.username.is_none());
}

#[test]
fn instance_deserialization_succeeds_with_required_fields() {
    let instance: Instance = serde_yaml::from_str("username: myuser\nimage:\n  type: distro\n  name: Ubuntu\n")
        .expect("deserialize instance");

    // name is serde-skipped; it will be empty when deserialized directly (set by loader from map key)
    assert!(instance.name.is_empty());
    assert_eq!(instance.username.as_deref(), Some("myuser"));
    assert_eq!(instance.install_dir, Path::new("%userprofile%/VMs"));
    assert!(!instance.override_instance);
    assert!(instance.cloud_init.is_none());

    match instance.image {
        ImageSource::Distro { name } => assert_eq!(name, "Ubuntu"),
        ImageSource::File { .. } => panic!("expected distro image"),
    }
}

#[test]
fn install_dir_defaults_to_userprofile_vms_when_omitted() {
    let instance: Instance = serde_yaml::from_str("username: myuser\nimage:\n  type: distro\n  name: Ubuntu\n")
        .expect("deserialize instance");
    assert_eq!(instance.install_dir, Path::new("%userprofile%/VMs"));
}

#[test]
fn cloud_init_input_display_matches_variant() {
    let file_input: CloudInitSource = serde_yaml::from_str("type: file\n").expect("deserialize file variant");
    assert_eq!(file_input.to_string(), "file: cloud-init.yaml");

    let inline_input: CloudInitSource =
        serde_yaml::from_str("type: inline\ncontent: '#cloud-config'\n").expect("deserialize inline variant");
    assert_eq!(inline_input.to_string(), "inline");
}
