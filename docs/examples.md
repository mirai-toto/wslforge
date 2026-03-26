# Examples

Ready-to-use configs are available in [`configs/`](../configs/). Copy one, adjust the fields you need, and run:

```sh
./wslforge --config config.dev.yaml
```

---

## config.dev.yaml — Ubuntu dev instance

Sets up a full Ubuntu development environment with:

- **Docker** (with Compose plugin)
- **MicroK8s** + kubectl
- **Java** (OpenJDK 21), **Node.js**, **npm**, **Python 3**
- systemd enabled, WSL metadata automount, and `~/.local/bin` on `PATH`
- Proxy-aware: if a proxy is configured, apt, git, and runcmd commands are all set up automatically

## config.vpnkit.yaml — same, with VPN support

Identical to `config.dev.yaml`, plus a [`wsl-vpnkit`](https://github.com/sakai135/wsl-vpnkit) sidecar instance for environments where WSL networking is blocked by a corporate VPN.

Provisions two instances:

| Instance | Purpose |
| --- | --- |
| `UbuntuWslDev` | The main dev environment |
| `wsl-vpnkit` | VPN proxy sidecar (downloaded automatically) |

The `UbuntuWslDev` instance registers `wsl-vpnkit` as a systemd service so it starts automatically.
