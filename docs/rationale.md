# 💡 Rationale

## Why this project exists

- Set up WSL the same way every time using one config file
- No more clicking around or doing steps by hand
- Share your setup with others easily
- Recreate your dev environment in minutes
- Break things safely and rebuild fast

## Why it's written in Rust

Originally prototyped in **PowerShell**, but moved to **Rust** for long-term reliability and maintainability.

- A single executable is easier for users than running and trusting scripts
- Strong typing and solid tooling make the app more reliable as it grows
- Great ecosystem for CLI apps, config parsing, logging, and testing

## Why not use an ISO image

Installing from an ISO is a valid approach — and not only are the two not mutually exclusive, they are actually meant to be used together. The intended workflow combines both: use the profile and cloud-init config to build and configure an instance first, then export the result as a rootfs to use as a versioned base image. The config is the source of truth; the image is the distributable artifact.

Keeping the instance definition as code — a small YAML profile and a cloud-init file — gives you:

- **Reproducibility** — run the same config on any machine and get the same result, with or without a custom base image
- **Transparency** — the full intent of the instance is readable in plain text, not locked inside an opaque image
- **Version control** — track changes to your environment the same way you track changes to application code
- **Composability** — profiles are easy to parameterise, share, and layer using Jinja templating

The relationship mirrors Docker: the config and cloud-init are your Dockerfile, and the ISO is the image snapshot of the result.

## Why it uses cloud-init

Originally provisioned with **Ansible**, but moved to **cloud-init** to better match first-boot, zero-prep environments.

- Simpler model: one config applied automatically at first boot
- Faster path to a ready system since provisioning happens during startup
- No prior network or SSH setup required to begin configuration
- Already included on most modern distros, no extra install step
