FROM rust:bookworm AS ci-base

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      git \
    && rm -rf /var/lib/apt/lists/*

# Use Node 20 to match GitHub Actions setup-node@v4 in CI.
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install beta nightly \
    && rustup component add rustfmt clippy --toolchain stable \
    && rustup component add rustfmt clippy --toolchain beta \
    && rustup component add rustfmt clippy --toolchain nightly

RUN cargo install --locked cargo-audit cargo-llvm-cov typos-cli

RUN npm install -g \
      markdownlint-cli2 \
      semantic-release \
      @semantic-release/commit-analyzer \
      @semantic-release/release-notes-generator \
      @semantic-release/exec \
      @semantic-release/github

WORKDIR /work
COPY . /work

# Rust compilation matrix checks
FROM ci-base AS ci-rust-matrix
RUN cargo +stable check --all-targets --all-features
RUN cargo +beta check --all-targets --all-features
RUN cargo +nightly check --all-targets --all-features

# Stable-only quality and build checks
FROM ci-rust-matrix AS ci-rust-quality
RUN cargo +stable fmt --all -- --check
RUN cargo +stable clippy --all-targets --all-features -- -D warnings
RUN cargo +stable test --all-features
RUN cargo +stable build --release
RUN cargo +stable doc --no-deps

# Compatibility alias used by scripts/run-local-ci.sh default target
FROM ci-rust-quality AS ci-core

# Markdown linting
FROM ci-core AS ci-markdown
RUN git ls-files '*.md' | xargs -r markdownlint-cli2

# Typo checks
FROM ci-markdown AS ci-typos
RUN git ls-files | xargs -r typos

# Security audit
FROM ci-typos AS ci-security-audit
RUN cargo audit

# Coverage reporting
FROM ci-security-audit AS ci-coverage
RUN cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info --summary-only
RUN cargo llvm-cov --all-features --workspace --cobertura --output-path cobertura.xml
RUN cargo llvm-cov report --summary-only

# semantic-release dry-run
FROM ci-coverage AS ci-semantic-release
RUN set -eux; \
  remote_url="$(git remote get-url origin)"; \
  case "$remote_url" in \
    git@github.com:*) \
      repo_path="${remote_url#git@github.com:}"; \
      git remote set-url origin "https://github.com/${repo_path}"; \
      ;; \
  esac

RUN GITHUB_TOKEN="local-dry-run" \
    GH_TOKEN="local-dry-run" \
    semantic-release --dry-run --no-ci

# Compatibility alias used by scripts/run-local-ci.sh --all target
FROM ci-semantic-release AS ci-all
