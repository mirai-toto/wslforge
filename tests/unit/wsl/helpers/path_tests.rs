use super::{expand_env_vars, expand_path, resolve_install_dir};
use std::path::Path;

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
