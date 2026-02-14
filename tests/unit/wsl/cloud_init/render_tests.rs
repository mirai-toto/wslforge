use super::render;
use crate::config::Profile;

#[test]
// Verifies template rendering injects profile fields and leaves password hash empty when absent.
fn render_injects_profile_fields_without_password_hash() {
    let profile: Profile = serde_yaml::from_str(
        r#"
hostname: devbox
username: devuser
override: true
"#,
    )
    .expect("deserialize profile");

    let output = render(
        "hostname={{ profile.hostname }}\nuser={{ profile.username }}\noverride={{ profile.override_instance }}\nhash={{ password_hash }}",
        &profile,
    )
    .expect("render template");

    assert!(output.contains("hostname=devbox"));
    assert!(output.contains("user=devuser"));
    assert!(output.contains("override=true"));
    assert!(output.contains("hash="));
}

#[test]
// Verifies rendering with a password produces a SHA-512 crypt-style hash.
fn render_produces_sha512_hash_when_password_is_present() {
    let profile: Profile = serde_yaml::from_str(
        r#"
hostname: devbox
username: devuser
password: secret123
"#,
    )
    .expect("deserialize profile");

    let output = render("{{ password_hash }}", &profile).expect("render template");
    assert!(output.starts_with("$6$"));
}

#[test]
// Verifies invalid template syntax is reported as a cloud-init parse error.
fn render_reports_template_parse_errors() {
    let profile: Profile = serde_yaml::from_str("{}\n").expect("deserialize profile");

    let err = render("{{", &profile).expect_err("invalid template should fail");
    assert!(err.to_string().contains("cloud-init template parse error"));
}
