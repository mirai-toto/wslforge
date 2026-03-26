#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  echo "Usage: run-local-ci.sh <mode>"
  echo ""
  echo "  --core      Run core checks via Docker"
  echo "  --all       Run all checks via Docker"
  echo "  --fast      Run core checks directly (no Docker)"
  echo "  --fast-all  Run all checks directly (no Docker)"
  exit 1
}

run_fast() {
  local extra="${1:-}"
  bash "$ROOT_DIR/scripts/ci-checks.sh" $extra
}

run_docker() {
  local target="$1"
  docker build \
    -f "$ROOT_DIR/scripts/ci-local.Dockerfile" \
    --target "$target" \
    "$ROOT_DIR"
}

case "${1:-}" in
  --core)     run_docker ci-core ;;
  --all)      run_docker ci-all ;;
  --fast)     run_fast ;;
  --fast-all) run_fast --all ;;
  *)          usage ;;
esac
