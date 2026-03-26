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
      @commitlint/cli \
      @commitlint/config-conventional \
      markdownlint-cli2 \
      semantic-release \
      @semantic-release/commit-analyzer \
      @semantic-release/release-notes-generator \
      @semantic-release/exec \
      @semantic-release/git \
      @semantic-release/github

WORKDIR /work
COPY . /work

# Core checks
FROM ci-base AS ci-core
RUN bash scripts/ci-checks.sh

# Extended checks
FROM ci-core AS ci-all
RUN bash scripts/ci-checks.sh --all
