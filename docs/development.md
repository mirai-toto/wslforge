# 🛠 Development

## Build locally

```sh
cargo build --release
cp configs/config.template.yaml config.yaml
./target/release/wslforge.exe --config config.yaml
```

## Git hooks

Enable the repo githooks and make the hook executable:

```sh
chmod +x .githooks/pre-commit
git config core.hooksPath .githooks
```

The pre-commit hook runs:

```sh
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --all-features
```

No extra tools required — only a stable Rust toolchain.

## Local CI

Two modes are available: fast (no Docker) and Docker-based.

### Fast mode (no Docker)

Runs checks directly on your machine. Requires the following tools to be installed:

| Tool                         | Install                                                          |
| ---------------------------- | ---------------------------------------------------------------- |
| Rust stable + beta + nightly | `rustup toolchain install stable beta nightly`                   |
| `cargo-audit`                | `cargo install cargo-audit`                                      |
| `cargo-llvm-cov`             | `cargo install cargo-llvm-cov`                                   |
| `typos-cli`                  | `cargo install typos-cli`                                        |
| `markdownlint-cli2`          | `npm install -g markdownlint-cli2`                               |
| `commitlint`                 | `npm install -g @commitlint/cli @commitlint/config-conventional` |

```sh
./scripts/run-local-ci.sh --fast      # core checks (fmt, clippy, check, test)
./scripts/run-local-ci.sh --fast-all  # all checks (core + beta/nightly, build, doc, markdownlint, typos, audit, commitlint, coverage)
```

### Docker mode

Runs the same checks inside a container. Only requires `docker`.

```sh
./scripts/run-local-ci.sh --core  # core checks
./scripts/run-local-ci.sh --all   # all checks
```

All check logic lives in `scripts/ci-checks.sh`, called by `scripts/ci-local.Dockerfile`.
