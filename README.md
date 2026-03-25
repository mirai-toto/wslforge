# ⚒️ wslforge

## 🔎 Overview

A minimal tool to declaratively create and manage WSL instances.

<!-- markdownlint-disable-next-line MD033 -->
<img src="./docs/assets/wslforge.svg" width="100px" align="left" alt="wslforge logo">

### WSL instance manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![build release artifacts](https://github.com/mirai-toto/wslforge/actions/workflows/build.yml/badge.svg)](https://github.com/mirai-toto/wslforge/actions/workflows/build.yml)
[![Latest Release](https://img.shields.io/github/v/release/mirai-toto/wslforge)](https://github.com/mirai-toto/wslforge/releases)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

✨ A clean, declarative way to create WSL instances from a single YAML config, with a focus on clarity and repeatability.

`wslforge` does two things:

1. **Manages WSL instances** — creates, configures, and optionally replaces instances from a YAML profile.
2. **Templates cloud-init user-data with Jinja** — your `cloud-init` content (file or inline) is rendered as a [Jinja](https://jinja.palletsprojects.com/) template before being handed to WSL, giving you variables, conditionals, and filters inside your user-data.

> Status: early/in-development. Some operations are still mock.

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

Download the latest release binary from: [Releases page](https://github.com/mirai-toto/wslforge/releases) 📦

```sh
curl -L -o wslforge.exe https://github.com/mirai-toto/wslforge/releases/download/v1.12.0/wslforge.exe
```

No config yet? Just run `wslforge` and the interactive wizard will guide you through creating one instance. It covers the essentials — hostname, user, image, proxy, and cloud-init. For advanced options like file transfers, scripts, or multi-instance setups, use a config file. 🧙

```sh
./wslforge
```

Already have a config? Run it directly: ✅

```sh
./wslforge --config config.yaml
```

Want to preview what will happen without making changes? Use dry-run: 🔍

```sh
./wslforge --config config.yaml --dry-run
```

Write a detailed log to a file for troubleshooting: 🧰

```sh
./wslforge --config config.yaml --log-file wslforge.log
```

---

## 🧭 CLI

| Flag                     | Description                                                    | Default |
| ------------------------ | -------------------------------------------------------------- | ------- |
| `--config`               | Path to YAML config file (optional, defaults to `config.yaml`) | —       |
| `--dry-run`              | Show what would be done without changes                        | `false` |
| `--debug`                | Enable extra debug output and write artifacts                  | `false` |
| `--log-file`             | Write debug logs with timestamps to a file                     | —       |
| `--print-example-config` | Print a minimal example config and exit                        | `false` |

If `--config` is omitted and no `config.yaml` is found in the current directory, an interactive wizard launches to build a config on the fly.

The `--debug` flag writes the rendered cloud-init user-data to the current directory as `cloud-init.<hostname>.user-data`.

```sh
./wslforge --print-example-config
```

---

## 📚 Learn more

- [🧩 Configuration](docs/configuration.md) — config schema, all fields, proxy, cloud-init, image sources, file transfers, post-create scripts
- [🧱 Architecture](docs/architecture.md) — how the CLI, provisioner, and engine fit together
- [💡 Rationale](docs/rationale.md) — why this project exists and the design decisions behind it
- [🛠 Development](docs/development.md) — building locally, git hooks, and running CI checks

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
