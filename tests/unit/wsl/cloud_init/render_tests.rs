use super::render;
use crate::config::Instance;

#[test]
// Verifies template rendering injects instance fields and leaves password hash empty when absent.
fn render_injects_instance_fields_without_password_hash() {
    let instance: Instance = serde_yaml::from_str(
        r#"
hostname: devbox
username: devuser
override: true
"#,
    )
    .expect("deserialize instance");

    let output = render(
        "hostname={{ instance.hostname }}\nuser={{ instance.username }}\noverride={{ instance.override_instance }}\nhash={{ password_hash }}",
        &instance,
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
    let instance: Instance = serde_yaml::from_str(
        r#"
hostname: devbox
username: devuser
password: secret123
"#,
    )
    .expect("deserialize instance");

    let output = render("{{ password_hash }}", &instance).expect("render template");
    assert!(output.starts_with("$6$"));
}

#[test]
// Verifies invalid template syntax is reported as a cloud-init parse error.
fn render_reports_template_parse_errors() {
    let instance: Instance = serde_yaml::from_str("{}\n").expect("deserialize instance");

    let err = render("{{", &instance).expect_err("invalid template should fail");
    assert!(err.to_string().contains("cloud-init template parse error"));
}
