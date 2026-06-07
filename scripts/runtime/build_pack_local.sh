#!/usr/bin/env bash
set -euo pipefail

echo "=== PROMETHEUS LOCAL BUILD + PACK ==="

echo
echo "=== TOOLCHAIN ==="
holochain --version
hc --version
lair-keystore --version
node --version

echo
echo "=== RUST CHECK ==="
cargo check

echo
echo "=== WASM BUILD ==="
cargo build --target wasm32-unknown-unknown

echo
echo "=== PREPARE DNA WASM ==="
mkdir -p dnas/hearth/wasm

cp target/wasm32-unknown-unknown/debug/zome_integrity.wasm \
   dnas/hearth/wasm/zome_integrity.wasm

cp target/wasm32-unknown-unknown/debug/zome_coordinator.wasm \
   dnas/hearth/wasm/zome_coordinator.wasm

echo
echo "=== CLEAN OLD ARTIFACTS ==="
rm -f dnas/hearth/hearth.dna hearth_prometheus.happ hearth_prometheus_web.webhapp ui.zip

echo
echo "=== PACK UI ==="
(cd ui && zip -r ../ui.zip .)

echo
echo "=== PACK DNA / HAPP / WEBHAPP ==="
hc dna pack dnas/hearth
hc app pack .
hc web-app pack .

echo
echo "=== GENERATED ARTIFACTS ==="
ls -lh \
  dnas/hearth/hearth.dna \
  hearth_prometheus.happ \
  hearth_prometheus_web.webhapp \
  ui.zip

echo
echo "BUILD_PACK_LOCAL: PASSED"
