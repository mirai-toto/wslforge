# wslforge

WSL instance manager driven by a YAML configuration file.

Status: early/in-development. Many operations are still mock.

## Features

- Parse a YAML config to describe a WSL instance.
- Dry-run mode to preview planned actions.
- Basic logging with verbosity flags.

## Install

```sh
cargo build --release
```

## Usage

```sh
./target/release/wslforge --config config.yaml
```

Dry run:

```sh
./target/release/wslforge --config config.yaml --dry-run
```

Verbosity:

```sh
./target/release/wslforge -v
./target/release/wslforge -vv
```

## Configuration

Copy the template and edit as needed:

```sh
cp config.template.yaml config.yaml
```

Fields (summary):

- `hostname`: WSL hostname
- `username`: default user
- `password`: optional; omit to disable password setup
- `http_proxy`, `https_proxy`, `no_proxy`: optional proxy settings
- `install_dir`: target install directory
- `cloud_init`: cloud-init file path
- `image`: source for the distro (official or local file)

## Development

```sh
cargo run -- --config config.yaml --dry-run -vv
```

## License

MIT
