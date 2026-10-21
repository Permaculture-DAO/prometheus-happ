# PROMETHEUS MRF Kernel Lab v0.1 — SYNTHETIC ONLY

This is an isolated reference laboratory crate. It is intentionally **not** listed in the main Cargo workspace, not linked to current integrity or coordinator zomes and not part of a signed runtime. This is not a production feature.

Controlling method candidate: [PROMETHEUS MRF Source — PR #29](https://github.com/Permaculture-DAO/prometheus-canon/pull/29), source ID `PRM-SRC-MRF-ZOME-EDGE-001`.

## What is implemented
- Pure Rust signed-integer, multi-output forest evaluation, with i128 accumulation and truncation toward zero.
- Frozen toy forest and synthetic outputs, without real ecological data.
- Candidate PMRF-BIN-v1 byte serialization and strict deserialization, explicit feature/tree/node/depth bounds, tree reachability and malformed-input rejection.
- 13 unit tests (golden vectors, threshold boundary, invalid input, malformed tree, negative division and native ABI smoke).
- Two synthetic C ABI exports `prometheus_mrf_lab_y0` / `prometheus_mrf_lab_y1` for WASM build smoke testing.

## Verified on 2026-10-19 (local WSL experiment)
`rustc 1.96.0 (ac68faa20 2026-05-25)`.

`cargo test --offline --manifest-path experimental/mrf-kernel-lab/Cargo.toml` → **13 passed, 0 failed** on local working copy.

`cargo build --offline --release --target wasm32-unknown-unknown --manifest-path experimental/mrf-kernel-lab/Cargo.toml` → **built successfully** on local working copy.

Local `.wasm` file: 28,681 bytes. Local SHA-256 `e206d5c0557ed22d93500a2cd6fc847df0e0c635cd2f0a03c61e98d9e49caf48`. This is **not** a signed or reproduced release hash.

## NOT run / NOT verified
- No execution of the emitted WASM module in a WASM runner, no native/WASM output equivalence proof.
- No Holochain `validate(Op)` callback or conductor sweettests; no package dependency on HDI/HDK.
- No content-addressed model registry or signed oracle attestations.
- No ecological scientific validity, adversarial oracle independence, sensor calibration, edge-power measurement, anti-gaming field validation.
- No changes to REAL capture, persistence, PRU/RAP/TRBK, production DNA or write paths.

## Required next gates
Reproduce build and hash independently; property/fuzz tests; runtime native/WASM equivalence; independent model/serializer review; version-pin HDI 0.7.1/Holochain 0.6.1 compatibility; implement synthetic-only integration in a separate experimental integrity zome; run exact-DNA conductor tests and explicitly authorize any migration. Preserve `REAL_DATA_PERSISTENCE_GATE_CLOSED`.

DO NOT merge, deploy or treat this as a validated ecological model merely because the native unit tests pass.
