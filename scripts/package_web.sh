#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="/Users/kevapatel/Developer/paypal/LockSmith"
OUT_DIR="$REPO_ROOT/platforms/web/pkg"

# Ensure wasm-pack is available
if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack is not installed. Install with: cargo install wasm-pack"
  exit 1
fi

# Clean previous outputs
rm -rf "$OUT_DIR"
rm -rf "$REPO_ROOT/pkg"
mkdir -p "$(dirname "$OUT_DIR")"

pushd "$REPO_ROOT" >/dev/null

# Build for bundlers (Vite/Webpack/Rollup). Use 'nodejs' or 'web' if needed.
wasm-pack build \
  --release \
  --target bundler \
  --features js

popd >/dev/null

# Move default output (REPO_ROOT/pkg) to platforms/javascript/pkg
mv "$REPO_ROOT/pkg" "$OUT_DIR"

echo "Web/WASM package built at: $OUT_DIR"


