#!/bin/sh
# Builds csvedit and packages it as an installable plank plugin.
#
# A plugin is a directory with `.plank-plugin/plugin.json` naming its modules,
# and the modules live under `wasm/`. The manifest is source (`plugin.json`):
# it declares the component's id, surfaces and capability grants.
#
# Needs the wasm32-wasip1 target (`rustup target add wasm32-wasip1`): Turbo
# Vision reads the clock, which needs WASI.
#
# Output: dist/csvedit/ (the installable directory), dist/plank-tv.tar.gz,
# and dist/SHA256SUMS with the module hash the trust store keys on, next to the
# rustc version the bytes are reproducible for.
set -e
cd "$(dirname "$0")"
ROOT=$(pwd)
DIST="$ROOT/dist"
rm -rf "$DIST"
mkdir -p "$DIST/csvedit/.plank-plugin" "$DIST/csvedit/wasm"

cargo build --release --target wasm32-wasip1
cp plugin.json "$DIST/csvedit/.plank-plugin/plugin.json"
cp target/wasm32-wasip1/release/plank_tv.wasm "$DIST/csvedit/wasm/csvedit.wasm"
tar -czf "$DIST/plank-tv.tar.gz" -C "$DIST" csvedit

{
  echo "# plank-tv module"
  echo "# $(rustc --version)"
  (cd "$DIST/csvedit/wasm" && shasum -a 256 csvedit.wasm)
} > "$DIST/SHA256SUMS"
echo "plugin: $DIST/csvedit"
cat "$DIST/SHA256SUMS"
