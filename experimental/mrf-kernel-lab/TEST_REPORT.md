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
