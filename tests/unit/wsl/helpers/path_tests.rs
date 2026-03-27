use super::{expand_env_vars, expand_path, expand_wsl_dest, resolve_install_dir};
use std::path::Path;

#[test]
fn expand_wsl_dest_tilde_only_regular_user() {
    assert_eq!(expand_wsl_dest("~", "alice", Path::new("file")), "/home/alice");
}

#[test]
fn expand_wsl_dest_tilde_slash_regular_user() {
    assert_eq!(
        expand_wsl_dest("~/.local/bin", "alice", Path::new("file")),
        "/home/alice/.local/bin"
    );
}

#[test]
fn expand_wsl_dest_tilde_only_root() {
    assert_eq!(expand_wsl_dest("~", "root", Path::new("file")), "/root");
}

#[test]
fn expand_wsl_dest_tilde_slash_root() {
    assert_eq!(
        expand_wsl_dest("~/.local/bin", "root", Path::new("file")),
        "/root/.local/bin"
    );
}

#[test]
fn expand_wsl_dest_absolute_path_unchanged() {
    assert_eq!(expand_wsl_dest("/etc/motd", "alice", Path::new("file")), "/etc/motd");
}

#[test]
fn expand_wsl_dest_trailing_slash_appends_filename() {
    assert_eq!(
        expand_wsl_dest("~/.local/bin/", "alice", Path::new("tproxy-deployment")),
        "/home/alice/.local/bin/tproxy-deployment"
    );
}

#[test]
fn expand_wsl_dest_trailing_slash_absolute_appends_filename() {
    assert_eq!(
        expand_wsl_dest("/usr/local/bin/", "alice", Path::new("/tmp/mytool")),
        "/usr/local/bin/mytool"
    );
}

#[test]
fn expand_env_vars_supports_percent_and_dollar_styles() {
    let key = "WSLFORGE_PATH_TEST_VAR";
    std::env::set_var(key, "value123");

    let expanded = expand_env_vars("%WSLFORGE_PATH_TEST_VAR%/$WSLFORGE_PATH_TEST_VAR").expect("expand env vars");
    assert_eq!(expanded, "value123/value123");
}

#[test]
fn resolve_install_dir_expands_and_appends_hostname() {
    let key = "WSLFORGE_INSTALL_ROOT";
    std::env::set_var(key, "/tmp/wslforge-root");

    let resolved = resolve_install_dir(Path::new("$WSLFORGE_INSTALL_ROOT"), "devbox").expect("resolve dir");
    assert_eq!(resolved, Path::new("/tmp/wslforge-root/devbox"));

    let expanded_path = expand_path(Path::new("%WSLFORGE_INSTALL_ROOT%/nested")).expect("expand path");
    assert_eq!(expanded_path, Path::new("/tmp/wslforge-root/nested"));
}
