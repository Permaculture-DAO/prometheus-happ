# Prometheus hApp Build Notes

## Prerequisites
- Rust toolchain installed
- target `wasm32-unknown-unknown`
- `hc` CLI available
- `lair-keystore` available

## Build WASM zomes
cargo build --release --target wasm32-unknown-unknown -p zome_integrity -p zome_coordinator

## Copy WASM into DNA bundle structure
mkdir -p dnas/hearth/wasm
cp target/wasm32-unknown-unknown/release/zome_integrity.wasm dnas/hearth/wasm/
cp target/wasm32-unknown-unknown/release/zome_coordinator.wasm dnas/hearth/wasm/

## Pack DNA
mkdir -p workdir/dna workdir/app
hc dna pack dnas/hearth -o workdir/dna/hearth.dna

## Pack hApp
Current stable path:
cp workdir/dna/hearth.dna dnas/hearth/hearth.dna
zip -j workdir/app/hearth_prometheus.happ happ.yaml dnas/hearth/hearth.dna

## Artifact sync
Artifacts are copied to:
D:\hearth intelligence – Prometheus Sovereign Syntropy\documenti\Holochain\artifacts

## Signatures
After artifact replacement, regenerate:
- SHA256_SUMS_HOLOCHAIN.txt
- per-file .sha256
- detached signature .asc

## Note
`hc app pack` under the current local CLI/tooling does not produce the preferred distributable artifact format for this workflow.
