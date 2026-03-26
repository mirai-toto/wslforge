#!/usr/bin/env bash
set -euo pipefail

ALL=false
if [[ "${1:-}" == "--all" ]]; then
  ALL=true
fi

section() { echo; echo "── $* ──────────────────────────────────────────────"; }

section "fmt"
cargo +stable fmt --all -- --check

section "clippy"
cargo +stable clippy --all-targets --all-features -- -D warnings

section "check (stable)"
cargo +stable check --all-targets --all-features

section "test"
cargo +stable test --all-features

if $ALL; then
  section "check (beta)"
  cargo +beta check --all-targets --all-features

  section "check (nightly)"
  cargo +nightly check --all-targets --all-features

  section "build"
  cargo +stable build --release

  section "doc"
  cargo +stable doc --no-deps

  section "markdownlint"
  git ls-files '*.md' | xargs -r markdownlint-cli2

  section "typos"
  typos

  section "audit"
  cargo audit

  section "commitlint"
  commitlint --from "$(git merge-base HEAD main)"

  section "coverage"
  rustup component add llvm-tools-preview --toolchain stable
  cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info --summary-only
  cargo llvm-cov --all-features --workspace --cobertura --output-path cobertura.xml
  cargo llvm-cov report --summary-only
fi

echo
echo "✅ all checks passed"
