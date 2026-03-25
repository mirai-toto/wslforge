# 🛠 Development

## Build locally

```sh
cargo build --release
cp config.template.yaml config.yaml
./target/release/wslforge --config config.yaml
```

## Git hooks

Enable the repo githooks and make the hook executable:

```sh
chmod +x .githooks/pre-commit
git config core.hooksPath .githooks
```

The pre-commit hook runs `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo check --all-targets --all-features`, and `cargo test --all-features`.

## Local CI

```sh
# Default checks (Docker target: ci-core)
./scripts/run-local-ci.sh

# Extended checks (Docker target: ci-all)
./scripts/run-local-ci.sh --all
```

Container prerequisite: `docker`.

All check logic lives in `scripts/ci-local.Dockerfile`. If the image build succeeds, checks passed.
Intermediate targets are available for focused runs: `ci-rust-matrix`, `ci-rust-quality`, `ci-markdown`, `ci-typos`, `ci-security-audit`, `ci-coverage`, `ci-semantic-release`.
