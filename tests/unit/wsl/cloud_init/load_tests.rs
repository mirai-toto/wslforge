use super::load_cloud_init_source;
use std::fs;

#[test]
// Verifies file-based cloud-init input reads and returns file contents.
fn load_cloud_init_source_reads_from_file_path() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let file_path = dir.path().join("user-data.yaml");
    fs::write(&file_path, "#cloud-config\nusers: []").expect("write cloud-init file");

    let loaded = load_cloud_init_source(&file_path).expect("load file cloud-init");
    assert_eq!(loaded, "#cloud-config\nusers: []");
}

#[test]
// Verifies a missing file path returns a clear not-found load error.
fn load_cloud_init_source_fails_for_missing_file() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let err =
        load_cloud_init_source(&dir.path().join("missing-cloud-init.yaml")).expect_err("missing file should fail");
    assert!(err.to_string().contains("cloud-init user-data file not found"));
}
