# PROMETHEUS MRF Milestone: isolated and repeatable runbook
Date: 2026-10-19 — status: laboratory / no production authority.

## Safety boundary
Only operate in experimental/mrf-kernel-lab. Never replace the signed genesis DNA,
existing integrity/coordinator zomes, production conductor configuration, Canon releases,
REAL_DATA_PERSISTENCE_GATE_CLOSED, or source-chain write paths. No tokens or capital.

## Tested platform
WSL/Linux, rustc 1.96.0, node v18.19.1, hc 0.6.1, HDK 0.6.1, HDI 0.7.1.
No pretrained or independently validated ecological MRF is included. All data are toy vectors.

## 1. Rust reference + actual WASM execution parity
Run from repository root:
    cargo test --offline --manifest-path experimental/mrf-kernel-lab/Cargo.toml
    cargo build --offline --release --bin mrf_vectors --manifest-path experimental/mrf-kernel-lab/Cargo.toml
    cargo build --offline --release --target wasm32-unknown-unknown --manifest-path experimental/mrf-kernel-lab/Cargo.toml
    node experimental/mrf-kernel-lab/wasm_parity_runner.cjs \
      experimental/mrf-kernel-lab/target/wasm32-unknown-unknown/release/prometheus_mrf_kernel_lab.wasm \
      experimental/mrf-kernel-lab/target/release/mrf_vectors

Expected: 4,108 native-generated vectors, 8,216 scalar WASM comparisons, PASS.
If build hashes differ, document compiler/flags and re-run parity. No silent hash equivalence.

## 2. Synthetic integrity zome host tests and WASM build
    (cd experimental/mrf-kernel-lab/zome_integrity_lab && cargo test --offline)
    (cd experimental/mrf-kernel-lab/zome_integrity_lab &&
       cargo build --offline --release --target wasm32-unknown-unknown)

The nested .cargo/config.toml pins WASM flags identical to the active hApp:
- getrandom_backend="unsupported": no consensus-critical entropy;
- linker --allow-undefined: Holochain __hc__ host functions stay WASM imports.

Host unit tests exercise the receipt's validation logic; NOT live DHT acceptance.

## 3. Synthetic-only coordinator and DNA
    (cd experimental/mrf-kernel-lab/zome_coordinator_lab &&
      cargo build --offline --release --target wasm32-unknown-unknown \
      --target-dir ../zome_integrity_lab/target)

    cp experimental/mrf-kernel-lab/zome_integrity_lab/target/wasm32-unknown-unknown/release/prometheus_mrf_integrity_lab.wasm \
      experimental/mrf-kernel-lab/experimental_dna/wasm/
    cp experimental/mrf-kernel-lab/zome_integrity_lab/target/wasm32-unknown-unknown/release/prometheus_mrf_coordinator_lab.wasm \
      experimental/mrf-kernel-lab/experimental_dna/wasm/
    hc dna pack experimental/mrf-kernel-lab/experimental_dna

Store the exact sha256sum of the two WASM files and experimental .dna file.
Never copy/overwrite into dnas/hearth/hearth.dna or a live conductor.

## 4. In-process conductor sweettests (separate PASS/FAIL gate)
The test manifest mirrors the exact pinned release deps of integration_tests/Cargo.toml.
Before --locked testing, obtain the same repository revision's integration_tests/Cargo.lock:
    cp integration_tests/Cargo.lock experimental/mrf-kernel-lab/conductor_tests/Cargo.lock

Then run:
    cargo test --offline --locked \
      --manifest-path experimental/mrf-kernel-lab/conductor_tests/Cargo.toml \
      --test conductor

Expected tests: accept valid TEST entry; reject corrupted outputs; reject REAL;
reject mismatched model hash; reject unauthorized financial admission.
Record pass/fail from the actual conductor test output, including loaded DNA digest.
Do not present this step as performed merely because the DNA packages successfully.

## 5. Known limits
- Toy model, not trained scientific MRF, never a regeneration certification.
- No proof of oracle independence/consensus, physical origin, ecological causality.
- No fuzzing or long-duration network partition/edge-energy measurements yet.
- A model change requires its own source/model hash governance and DNA review.
- Current validate callback checks StoreEntry CreateEntry/UpdateEntry. Explicit
  review/testing of other Holochain op types (StoreRecord, RegisterUpdate,
  RegisterDelete and dependencies) is mandatory before claiming complete action
  transition coverage or deployment readiness.
- Successful conductor tests do not alone demonstrate independent multi-peer DHT validation.

## 6. Required next gate
Independent build reproduction, executable conductor test evidence, adversarial
op coverage, no-double-counting and oracle independence tests, steward-controlled
DNA migration review. REAL persistence remains disallowed throughout this milestone.
