mod loader;
mod model;

pub use loader::load_yaml;
pub use model::{CloudInitSource, ImageSource, Profile, RootConfig};

pub const EXAMPLE_CONFIG: &str = r#"─── Example Config ───────────────────────────────────────────────

profiles:
  UbuntuWslDev:
    override: true
    hostname: UbuntuWslDev
    username: wsluser
    cloud_init:
      type: inline
      content: |
        #cloud-config
        users:
          - name: wsluser
            sudo: ALL=(ALL) NOPASSWD:ALL
    image:
      type: distro
      name: Ubuntu

─── Tips ─────────────────────────────────────────────────────────
• Use `--print-config` to print this example.
• Use `--config` to point to your YAML file.
"#;
