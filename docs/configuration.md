# 🧩 Configuration

The configuration is intentionally small. All fields are optional and have sensible defaults, so you can start with a minimal config and grow into advanced options as needed.

The top-level config is an `instances` map, where each key is an instance name:

```yaml
instances:
  MyInstance:
    hostname: MyInstance
    username: wsluser
```

Note: a bare instance object at the root (without `instances:`) is still accepted for backward compatibility, but the recommended format is the `instances` map.

## Core fields (per instance)

| Field         | Description                                    | Example / Reference                 | Default             |
| ------------- | ---------------------------------------------- | ----------------------------------- | ------------------- |
| `override`    | Replace existing instance if it exists         | `true`                              | `false`             |
| `hostname`    | WSL instance name                              | `UbuntuWslDev`                      | `UbuntuWSL`         |
| `username`    | Default user                                   | `wsluser`                           | `wsluser`           |
| `password`    | Optional password (hashed for cloud-init)      | `root`                              | —                   |
| `install_dir` | Target install directory                       | `%userprofile%/VMs`                 | `%userprofile%/VMs` |
| `proxy`       | HTTP/HTTPS proxy settings                      | see [Proxy](#proxy)                 | —                   |
| `image`       | Image source (distro or file/URL)              | see [Image Sources](#image-sources) | distro: Ubuntu      |
| `cloud_init`  | Cloud-init user-data (file or inline)          | see [Cloud Init](#cloud-init)       | —                   |
| `files`       | Files or directories to copy into the instance | see [Files](#files)                 | —                   |
| `scripts`     | Commands to run after create                   | see [Scripts](#scripts)             | —                   |

Example `config.yaml` with a file-based cloud-init and an official distro:

```yaml
instances:
  UbuntuWslDev:
    override: true
    hostname: UbuntuWslDev
    username: wsluser
    password: root

    proxy:
      http: http://proxy.local:8080
      https: http://proxy.local:8080
      no_proxy: localhost,127.0.0.1

    install_dir: "%userprofile%/VMs"

    cloud_init:
      type: file
      path: "cloud-init.yaml"

    image:
      type: distro
      name: Ubuntu
```

## Proxy

Configure HTTP/HTTPS proxy settings for the instance. All fields are optional.

```yaml
proxy:
  http: http://proxy.local:8080
  https: http://proxy.local:8080
  no_proxy: localhost,127.0.0.1
```

## Cloud init

Use cloud-init to bootstrap packages and settings on first boot. You can reference a file or embed the YAML inline. These blocks live inside an instance.

Both `file` and `inline` content are rendered as **Jinja templates** before being written as user-data. Instance fields are available as direct template variables (e.g. `{{ username }}`, `{{ hostname }}`), and the hashed password is available as `{{ password_hash }}`. Proxy fields are available as `{{ proxy.http }}`, `{{ proxy.https }}`, `{{ proxy.no_proxy }}`. Custom variables defined under `vars:` are accessible as `{{ vars.my_key }}`.

Cloud-init types:

| Type     | Description                | Example                   |
| -------- | -------------------------- | ------------------------- |
| `file`   | Load user-data from a file | `path: "cloud-init.yaml"` |
| `inline` | Inline YAML user-data      | `content: \| ...`         |

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

## Image Sources

Pick where the root filesystem comes from: an official WSL distro or a rootfs archive.

| Type     | Description                              | Example                               |
| -------- | ---------------------------------------- | ------------------------------------- |
| `distro` | Install from official WSL distro         | `name: Ubuntu`                        |
| `file`   | Import from a local path or a remote URL | `path: "%USERPROFILE%/Downloads/..."` |

Official WSL distro (simple and quick):

```yaml
image:
  type: distro
  name: Ubuntu
```

Local rootfs archive:

```yaml
image:
  type: file
  path: "%USERPROFILE%/Downloads/ubuntu-noble-wsl-amd64-ubuntu.rootfs.tar.gz"
```

Remote URL (downloaded automatically before import):

```yaml
image:
  type: file
  path: "https://github.com/sakai135/wsl-vpnkit/releases/latest/download/wsl-vpnkit.tar.gz"
```

## Files

Copy files or directories into the instance after creation. Useful for injecting certificates, config files, or scripts. If `src` points to a directory, the entire directory is transferred recursively.

```yaml
files:
  - src: "motd.txt"
    dest: /etc/motd
    mode: "644"
  - src: "%USERPROFILE%/certs/company.crt"
    dest: /usr/local/share/ca-certificates/company.crt
    owner: root
    mode: "644"
  - src: "%USERPROFILE%/dotfiles/config"
    dest: /home/user/.config
    owner: user
    mode: "755"
```

| Field   | Description                                         | Required |
| ------- | --------------------------------------------------- | -------- |
| `src`   | Local source path — file or directory (env vars OK) | yes      |
| `dest`  | Destination path inside the WSL instance            | yes      |
| `owner` | File/directory owner (e.g. `root`)                  | no       |
| `mode`  | File/directory permissions (e.g. `"644"`, `"755"`)  | no       |

**Path expansion in `dest`:**

- `~` is expanded to the instance user's home directory (e.g. `/home/user`)
- If `dest` ends with `/`, the source filename is appended — e.g. `dest: /etc/ssl/certs/` with `src: company.crt` writes to `/etc/ssl/certs/company.crt`

## Scripts

Run commands inside the instance after creation. Runs after file transfers.

```yaml
scripts:
  run:
    - "systemctl enable my-service"
    - "systemctl start my-service"
```

| Field   | Description                                    | Default |
| ------- | ---------------------------------------------- | ------- |
| `run`   | List of commands to execute                    | —       |
| `shell` | Shell used to run commands (e.g. `bash`, `sh`) | `sh`    |

Each entry is passed to `<shell> -c` inside the instance. The default shell is `sh`, which works on any distro including Alpine-based images. Override to `bash` if your scripts require it:

```yaml
scripts:
  shell: bash
  run:
    - "source /etc/profile && my-bash-script"
```
