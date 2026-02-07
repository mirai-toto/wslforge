# ⚒️ wslforge

## 🔎 Overview

A minimal tool to declaratively create and manage WSL instances.

<!-- markdownlint-disable-next-line MD033 -->
<img src="./docs/assets/wslforge.svg" width="100px" align="left" alt="wslforge logo">

### WSL instance manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Release builds](https://github.com/mirai-toto/wslforge/actions/workflows/release.yml/badge.svg?branch=main)](https://github.com/mirai-toto/wslforge/actions/workflows/release.yml)
[![Latest Release](https://img.shields.io/github/v/release/mirai-toto/wslforge)](https://github.com/mirai-toto/wslforge/releases)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

✨ A clean, declarative way to create WSL instances from a single YAML config, with a focus on clarity and repeatability.

> Status: early/in-development. Some operations are still mock.

---

## ✅ Requirements

WSL must be enabled on Windows before you can create instances. Run this once in an elevated PowerShell:

```powershell
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

Optional: install the latest PowerShell via winget if you prefer a newer shell experience:

```powershell
winget install --id Microsoft.PowerShell --source winget
```

---

## ⚡ Quickstart

Download the latest release binary from: 📦

```text
https://github.com/mirai-toto/wslforge/releases
```

Run it with your config: ✅

```sh
./wslforge --config config.yaml
```

Want to preview what will happen without making changes? Use dry-run: 🔍

```sh
./wslforge --config config.yaml --dry-run
```

Need more details for troubleshooting? Increase verbosity: 🧰

```sh
./wslforge -v
./wslforge -vv
```

---

## 🧭 CLI

Common flags:

| Flag | Description | Default |
| --- | --- | --- |
| `--config` | Path to YAML config file | `config.yaml` |
| `--dry-run` | Show what would be done without changes | `false` |
| `--debug` | Enable extra debug output and artifacts | `false` |
| `-v`, `-vv` | Increase verbosity | `0` |

---

## 🛠 Development

Build locally:

```sh
cargo build --release
cp config.template.yaml config.yaml
./target/release/wslforge --config config.yaml
```

Enable the repo githooks and make the hook executable:

```sh
chmod +x .githooks/pre-commit
git config core.hooksPath .githooks
```

The pre-commit hook runs `cargo fmt --all`.

---

## 🧩 Configuration

The configuration is intentionally small. Most fields are optional, and you can grow into advanced options as needed.

The top-level config is now a `profiles` map, where each key is a profile name:

```yaml
profiles:
  MyProfile:
    hostname: MyProfile
    username: wsluser
```

Note: a single-profile file without `profiles:` is still accepted for backward compatibility, but the recommended format is the `profiles` map.

Core fields (per profile):

| Field | Description | Example | Mandatory |
| --- | --- | --- | --- |
| `override` | Replace existing instance if it exists | `true` | ➖ |
| `hostname` | WSL instance name | `UbuntuWslDev` | ✅ |
| `username` | Default user | `wsluser` | ✅ |
| `password` | Optional password (hashed for cloud-init) | `root` | ➖ |
| `install_dir` | Target install directory | `%userprofile%/VMs` | ✅ |
| `http_proxy` | HTTP proxy URL | `http://proxy.local:8080` | ➖ |
| `https_proxy` | HTTPS proxy URL | `https://proxy.local:8443` | ➖ |
| `no_proxy` | Comma-separated proxy bypass list | `localhost,127.0.0.1` | ➖ |

Related sections:

- [🐧 Image source section](#image-sources)
- [☁️ Cloud init section](#cloud-init)

Example `config.yaml` with a file-based cloud-init and an official distro:

```yaml
profiles:
  UbuntuWslDev:
    override: true
    hostname: UbuntuWslDev
    username: wsluser
    password: root

    http_proxy: null
    https_proxy: null
    no_proxy: null

    install_dir: "%userprofile%/VMs"

    cloud_init:
      type: file
      path: "cloud-init.yaml"

    image:
      type: distro
      name: Ubuntu
```

### Cloud init

Use cloud-init to bootstrap packages and settings on first boot. You can reference a file or embed the YAML inline. These blocks live inside a profile.

Cloud-init types:

| Type | Description | Example |
| --- | --- | --- |
| `file` | Load user-data from a file | `path: "cloud-init.yaml"` |
| `inline` | Inline YAML user-data | `content: \| ...` |

File-based user-data (recommended for larger configs):

```yaml
cloud_init:
  type: file
  path: "cloud-init.yaml"
```

Inline user-data (handy for small, self-contained configs):

```yaml
cloud_init:
  type: inline
  content: |
    #cloud-config
    packages:
      - curl
```

### Image Sources

Pick where the root filesystem comes from: an official WSL distro or a local rootfs archive. These blocks live inside a profile.

Image types:

| Type | Description | Example |
| --- | --- | --- |
| `distro` | Install from official WSL distro | `name: Ubuntu` |
| `file` | Import from local rootfs archive | `path: "%USERPROFILE%/Downloads/..."` |

Official WSL distro (simple and quick):

```yaml
image:
  type: distro
  name: Ubuntu
```

Local rootfs archive (for custom or prebuilt images):

```yaml
image:
  type: file
  path: "%USERPROFILE%/Downloads/ubuntu-noble-wsl-amd64-ubuntu.rootfs.tar.gz"
```

---

## 📄 License

MIT — see [LICENSE](https://github.com/mirai-toto/wslforge/blob/main/LICENSE).

---

## 🤝 Support

Open an issue at [GitHub Issues](https://github.com/mirai-toto/wslforge/issues) with your logs and config details if possible.

---

## 🔗 Useful Links

- [Cloud-init WSL datasource](https://cloudinit.readthedocs.io/en/latest/topics/datasources/wsl.html) for user-data behavior and file locations
- [WSL documentation](https://learn.microsoft.com/windows/wsl/) for setup, commands, and troubleshooting

---

## 👤 Credits

Made by [mirai-toto](https://github.com/mirai-toto). Thanks for checking it out!

---

## 🙏 Acknowledgements

Thanks to the maintainers of WSL, cloud-init, Docker, k3s, Helm, and wsl-vpnkit.
