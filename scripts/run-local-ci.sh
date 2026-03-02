#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="ci-core"
if [[ "${1:-}" == "--all" ]]; then
  TARGET="ci-all"
fi

exec docker build \
  -f "$ROOT_DIR/scripts/ci-local.Dockerfile" \
  --target "$TARGET" \
  "$ROOT_DIR"
