use super::resolve_userprofile_dir;
use std::path::PathBuf;

#[test]
fn resolve_userprofile_dir_returns_env_path_when_set() {
    let _guard = super::userprofile_env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let original = std::env::var_os("USERPROFILE");
    std::env::set_var("USERPROFILE", "/tmp/wslforge-userprofile");

    let resolved = resolve_userprofile_dir().expect("USERPROFILE should resolve");
    assert_eq!(resolved, PathBuf::from("/tmp/wslforge-userprofile"));

    match original {
        Some(v) => std::env::set_var("USERPROFILE", v),
        None => std::env::remove_var("USERPROFILE"),
    }
}

#[test]
fn resolve_userprofile_dir_errors_when_missing() {
    let _guard = super::userprofile_env_lock().lock().unwrap_or_else(|e| e.into_inner());
    let original = std::env::var_os("USERPROFILE");
    std::env::remove_var("USERPROFILE");

    let err = resolve_userprofile_dir().expect_err("missing USERPROFILE should error");
    assert!(err.to_string().contains("USERPROFILE is not set"));

    match original {
        Some(v) => std::env::set_var("USERPROFILE", v),
        None => std::env::remove_var("USERPROFILE"),
    }
}
