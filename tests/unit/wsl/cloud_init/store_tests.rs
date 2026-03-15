use super::{copy_debug_to_current_dir, store, DebugCopyOutcome};
use std::fs;

#[test]
// Verifies `store` creates missing parent directories and writes the rendered content.
fn store_creates_parent_dirs_and_writes_file() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let target = dir.path().join("nested/cloud-init/user-data");

    store(&target, "hello cloud-init").expect("store should succeed");

    let written = fs::read_to_string(&target).expect("read written file");
    assert_eq!(written, "hello cloud-init");
}

#[test]
// Verifies debug copy writes `cloud-init.<hostname>.user-data` into the current working directory.
fn copy_debug_writes_file_in_current_directory() {
    let dir = tempfile::tempdir().expect("create temp dir");
    let original_dir = std::env::current_dir().expect("current dir");
    std::env::set_current_dir(dir.path()).expect("switch cwd");

    let outcome = copy_debug_to_current_dir("myhost", "rendered content");

    std::env::set_current_dir(original_dir).expect("restore cwd");

    match outcome {
        DebugCopyOutcome::Written(path) => {
            assert!(path.ends_with("cloud-init.myhost.user-data"));
            let content = fs::read_to_string(path).expect("read debug copy");
            assert_eq!(content, "rendered content");
        }
        DebugCopyOutcome::Skipped(reason) => panic!("unexpected skip: {reason}"),
    }
}
