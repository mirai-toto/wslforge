# ⚒️ wslforge

## 🔎 Overview

A minimal tool to declaratively create and manage WSL instances.

<!-- markdownlint-disable-next-line MD033 -->
<img src="./docs/assets/wslforge.svg" width="100px" align="left" alt="wslforge logo">

### WSL instance manager

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![build release artifacts](https://github.com/mirai-toto/wslforge/actions/workflows/build-windows.yml/badge.svg)](https://github.com/mirai-toto/wslforge/actions/workflows/build-windows.yml)
[![Latest Release](https://img.shields.io/github/v/release/mirai-toto/wslforge)](https://github.com/mirai-toto/wslforge/releases)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

✨ A clean, declarative way to create WSL instances from a single YAML config, with a focus on clarity and repeatability.

`wslforge` does two things:

1. **Manages WSL instances** — creates, configures, and optionally replaces instances from a YAML config.
2. **Automates instance setup** — drives [cloud-init](https://cloud-init.io) for automation and templating.

> Status: early/in-development. Some operations are still mock.

## ✅ Requirements

Optional: install the latest PowerShell via winget if you prefer a newer shell experience:

```powershell
winget install --id Microsoft.PowerShell --source winget
```

WSL must be enabled on Windows before you can create instances. Run this once in an elevated PowerShell:

```powershell
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
```

If you run into issues, a WSL update may be needed:

```powershell
wsl --update
```

---

## ⚡ Quickstart

![wslforge installation demo](./docs/assets/demo.gif)

### 1. Install

Download the latest release binary from the [Releases page](https://github.com/mirai-toto/wslforge/releases) 📦

```powershell
Invoke-WebRequest -Uri https://github.com/mirai-toto/wslforge/releases/download/v1.16.3/wslforge.exe -OutFile wslforge.exe
```

### 2. No config yet?

Run `wslforge` with no arguments and the interactive wizard will guide you through creating one instance. It covers the essentials — hostname, user, image, proxy, and cloud-init. For advanced options like file transfers, scripts, or multi-instance setups, use a config file. 🧙

```sh
./wslforge
```

Looking for inspiration? Check out the [ready-to-use examples](docs/examples.md).

### 3. Have a config?

Run it directly:

```sh
./wslforge --config config.yaml
```

Preview what will happen without making changes:

```sh
./wslforge --config config.yaml --dry-run
```

Write a detailed log to a file for troubleshooting:

```sh
./wslforge --config config.yaml --log-file wslforge.log
```

---

## 🧭 CLI

| Flag                     | Description                                                                              | Default |
| ------------------------ | ---------------------------------------------------------------------------------------- | ------- |
| `--config`               | Path to YAML config file (optional, defaults to `config.yaml`)                           | —       |
| `--dry-run`              | Show what would be done without changes                                                  | `false` |
| `--debug`                | Enable extra debug output and write artifacts                                            | `false` |
| `--log-file`             | Write debug logs with timestamps to a file                                               | —       |
| `--force`, `-f`          | Skip confirmation prompt and proceed automatically                                       | `false` |
| `--print-example-config` | Print a minimal example config and exit                                                  | `false` |
| `--generate-completion`  | Print a shell completion script and exit (`powershell`)                                  | —       |

If `--config` is omitted and no `config.yaml` is found in the current directory, an interactive wizard launches to build a config on the fly.

The `--debug` flag writes the rendered cloud-init user-data to the current directory as `cloud-init.<hostname>.user-data`.

```sh
./wslforge --print-example-config
```

### Shell completions

Generate and install a completion script for your shell:

```powershell
# PowerShell — load for the current session only
./wslforge --generate-completion powershell | Out-String | Invoke-Expression

# To persist across sessions, save to a separate file and dot-source it from your profile
./wslforge --generate-completion powershell | Out-File "$HOME\wslforge_completion.ps1"
Add-Content $PROFILE ". `"$HOME\wslforge_completion.ps1`""
```

---

## 📚 Learn more

- [🧩 Configuration](docs/configuration.md) — config schema, all fields, proxy, cloud-init, image sources, file transfers, post-create scripts
- [📋 Examples](docs/examples.md) — ready-to-use configs for common setups
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
