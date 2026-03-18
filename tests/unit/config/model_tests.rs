use super::{CloudInitSource, ImageSource, Instance};
use std::path::Path;

#[test]
fn instance_deserialization_applies_defaults() {
    let instance: Instance = serde_yaml::from_str("{}\n").expect("deserialize instance");

    assert!(!instance.override_instance);
    assert_eq!(instance.hostname, "UbuntuWSL");
    assert_eq!(instance.username, "wsluser");
    assert_eq!(instance.install_dir, Path::new("%userprofile%/VMs"));
    assert!(instance.cloud_init.is_none());

    match instance.image {
        ImageSource::Distro { name } => assert_eq!(name, "Ubuntu"),
        ImageSource::File { .. } => panic!("expected default distro image"),
    }
}

#[test]
fn cloud_init_input_display_matches_variant() {
    let file_input: CloudInitSource = serde_yaml::from_str("type: file\n").expect("deserialize file variant");
    assert_eq!(file_input.to_string(), "file: cloud-init.yaml");

    let inline_input: CloudInitSource =
        serde_yaml::from_str("type: inline\ncontent: '#cloud-config'\n").expect("deserialize inline variant");
    assert_eq!(inline_input.to_string(), "inline");
}
