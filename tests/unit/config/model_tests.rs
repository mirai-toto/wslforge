use super::{CloudInitSource, ImageSource, Profile};
use std::path::Path;

#[test]
fn profile_deserialization_applies_defaults() {
    let profile: Profile = serde_yaml::from_str("{}\n").expect("deserialize profile");

    assert!(!profile.override_instance);
    assert_eq!(profile.hostname, "UbuntuWSL");
    assert_eq!(profile.username, "wsluser");
    assert_eq!(profile.install_dir, Path::new("%userprofile%/VMs"));
    assert!(profile.cloud_init.is_none());

    match profile.image {
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
