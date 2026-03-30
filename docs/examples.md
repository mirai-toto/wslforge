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

| Instance       | Purpose                                      |
| -------------- | -------------------------------------------- |
| `UbuntuWslDev` | The main dev environment                     |
| `wsl-vpnkit`   | VPN proxy sidecar (downloaded automatically) |

The `UbuntuWslDev` instance registers `wsl-vpnkit` as a systemd service so it starts automatically.

## config.files.yaml — file and directory transfer

Shows how to copy files and directories into an instance after creation:

- A single file to `/etc/motd`
- The entire `configs/` directory into the instance

## config.scripts.yaml — post-create scripts

Shows how to run shell commands inside the instance after creation:

- Package installation as root
- A welcome message written to `/etc/profile.d/` (visible on every login)
- Per-user git config using `su -`

## config.files-and-scripts.yaml — file transfer + scripts combined

Shows the full post-create flow: files are transferred first, then scripts run against them.

Transfers `configs/setup.sh` into the instance, then executes it.

## config.vars.yaml — custom template variables

Shows how `vars` can be used to inject values into a cloud-init Jinja template without touching the template itself:

- Strings (`timezone`, `git_name`, `git_email`)
- Arrays (`packages` — iterated with `{% for %}`)
- Nested objects (`motd.title`, `motd.message`)

After provisioning, verify with:

```sh
cat /etc/wslforge-vars-test
```
