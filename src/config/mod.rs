mod loader;
mod model;
pub use loader::load_yaml;
pub use model::{CloudInitSource, Config, ImageSource, Instance, Proxy};

pub const EXAMPLE_CONFIG: &str = r#"# ─── Example Config ───────────────────────────────────────────────

instances:
  UbuntuWslDev:
    override: true
    hostname: UbuntuWslDev
    username: wsluser
    password: <PASSWORD>
    # proxy:
    #   http: http://proxy.local:8080
    #   https: http://proxy.local:8080
    #   no_proxy: localhost,127.0.0.1
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

# ─── Tips ─────────────────────────────────────────────────────────
# Use --print-example-config to print this example.
# Redirect to a file:  wslforge --print-example-config > config.yaml
# Use --config to point to your YAML file.
"#;
