# MRF Kernel v0.1 — Local Laboratory Test Report

Date: 2026-10-19. Status: LOCAL EXPERIMENT / NOT INDEPENDENTLY REPRODUCED.

Code: isolated Rust laboratory, no live hApp wiring.
Toolchain: rustc 1.96.0 (ac68faa20 2026-05-25).
Native tests: 13 passed / 0 failed / 0 ignored.
Test families: synthetic golden outputs; threshold strictness; trunc-toward-zero; serialization roundtrip; trailing/truncated bytes; malformed node tag; invalid tree index/orphan; invalid feature; wrong input shape; native C ABI smoke.

Build: `wasm32-unknown-unknown --release` succeeded, 28,681 bytes.
Local WASM SHA-256: `e206d5c0557ed22d93500a2cd6fc847df0e0c635cd2f0a03c61e98d9e49caf48`.

**NOT RUN:** WASM execution/parity, Holochain conductor, fuzzing, model hash runtime integrity, attestation quorum, field calibration, ecological/out-of-site validation, device energy, DHT UnresolvedDependencies.

Gate: lab kernel arithmetic GO; zome-side/REAL/production claims NO-GO pending the above.


---
## Stage Gate 0–30 — executed evidence (2026-10-19, federica WSL)

**PASS — Native/WASM runtime parity**: Node.js v18.19.1 instantiated the compiled `wasm32-unknown-unknown` module with zero imports and executed `prometheus_mrf_lab_y0/y1`. A native Rust binary independently generated 4,108 input vectors (12 edge vectors, 4,096 deterministic LCG vectors) and expected outputs using the same Rust reference crate. Exactly 8,216 scalar comparisons passed with **zero mismatches** (includes i64 extremes, negative inputs, threshold equality). Executed artifact: 28,681 bytes, SHA-256 `e206d5c0557ed22d93500a2cd6fc847df0e0c635cd2f0a03c61e98d9e49caf48`.

Reproduction (after building both artifacts):
```sh
cargo build --release --bin mrf_vectors --manifest-path experimental/mrf-kernel-lab/Cargo.toml
cargo build --release --target wasm32-unknown-unknown --manifest-path experimental/mrf-kernel-lab/Cargo.toml
node experimental/mrf-kernel-lab/wasm_parity_runner.cjs \
  experimental/mrf-kernel-lab/target/wasm32-unknown-unknown/release/prometheus_mrf_kernel_lab.wasm \
  experimental/mrf-kernel-lab/target/release/mrf_vectors
```

**PASS — Host Holochain integrity unit tests:** `zome_integrity_lab` using HDK 0.6.1 + HDI 0.7.1, 9/9 passed in WSL. They exercise synthetic acceptance and negative checks for forged outputs, wrong inputs, incorrect hash, TEST namespace, REAL rejection, non-authoritative claim and financial-admission rejection. This is **host execution of entry validation logic**, not evidence of conductor execution.

**PASS — Integrity zome WASM compilation:** compilation succeeded on isolated standalone crate using exactly the existing hApp WASM toolchain settings: `getrandom_backend="unsupported"` and linker `--allow-undefined` for Holochain host imports. The isolated binary is unsigned, not a signed DNA release. Frozen synthetic PMRF-BIN-v1 115-byte model SHA-256: `e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f`.

**PASS — Synthetic DNA packaging:** integrity/coordinator WASM isolated DNA bundle was created using `hc dna pack`; explicitly **not** the existing `hearth.dna`. A conductor sweettest run remains a separate gate; do not infer success from compilation, packaging, or host-only tests.

**OPEN:** conductor outcome, independent rebuild, fuzzing, deterministic model governance/admission, oracle independence, scientific validity, device energy. The REAL_DATA_PERSISTENCE_GATE_CLOSED is retained.


## Cross-environment CI confirmation — GitHub Actions (2026-10-19)

The first CI attempt exposed a Cargo workspace-isolation problem. It was corrected by declaring an empty workspace in the standalone kernel manifest; no main workspace packages or production logic changed.

The next GitHub Actions run [39325521772](https://github.com/Permaculture-DAO/prometheus-happ/actions/runs/39325521772) completed the **rust-wasm-parity** job successfully on an independent Ubuntu runner:
- 4,108 vectors, 8,216 i64 comparisons, zero mismatches;
- Node.js v22.23.3, WASM imports = 0, native process exited 0;
- independently compiled WASM: 29,244 bytes, SHA-256 ada00809503adfc57dfbed7aa4b930271c2788a90d2fdcd3df0c8e36fcdfb373.

Compared with the local WSL WASM (28,681 bytes; e206d5c0557ed22d93500a2cd6fc847df0e0c635cd2f0a03c61e98d9e49caf48), the binary hash and size differ under a different toolchain. **Numerical parity is demonstrated; byte-identical reproducible builds are NOT demonstrated.** Freeze compiler, flags and dependencies before signed release claims.

The first CI attempt also completed independent HDI host tests successfully. No independent multi-peer DHT tests or field/ecological validation are implied.
