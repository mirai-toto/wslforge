use super::load_cloud_init_source;
use crate::config::CloudInitInput;
use std::fs;

#[test]
fn load_cloud_init_source_returns_inline_content() {
    let source = CloudInitInput::Inline {
        content: "#cloud-config\nhostname: testbox".to_string(),
    };

    let loaded = load_cloud_init_source(&source).expect("load inline cloud-init");
    assert_eq!(loaded, "#cloud-config\nhostname: testbox");
}

#[test]
fn load_cloud_init_source_reads_from_file_path() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let file_path = dir.path().join("user-data.yaml");
    fs::write(&file_path, "#cloud-config\nusers: []").expect("write cloud-init file");

    let source = CloudInitInput::File { path: file_path };
    let loaded = load_cloud_init_source(&source).expect("load file cloud-init");
    assert_eq!(loaded, "#cloud-config\nusers: []");
}

#[test]
fn load_cloud_init_source_fails_for_missing_file() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let source = CloudInitInput::File {
        path: dir.path().join("missing-cloud-init.yaml"),
    };

    let err = load_cloud_init_source(&source).expect_err("missing file should fail");
    assert!(err.to_string().contains("cloud-init user-data file not found"));
}
