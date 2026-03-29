use super::{CloudInitSource, ImageSource, Instance};
use std::path::Path;

#[test]
fn instance_deserialization_requires_only_hostname_for_single_instance() {
    // username and image are optional — empty instance only fails at the loader level (no hostname to derive from)
    let instance: Instance = serde_yaml::from_str("{}\n").expect("bare instance should deserialize");
    assert!(instance.hostname.is_empty());
    assert!(instance.username.is_none());
}

#[test]
fn instance_deserialization_succeeds_with_required_fields() {
    let instance: Instance =
        serde_yaml::from_str("hostname: myhost\nusername: myuser\nimage:\n  type: distro\n  name: Ubuntu\n")
            .expect("deserialize instance");

    assert_eq!(instance.hostname, "myhost");
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
    let instance: Instance =
        serde_yaml::from_str("hostname: myhost\nusername: myuser\nimage:\n  type: distro\n  name: Ubuntu\n")
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
