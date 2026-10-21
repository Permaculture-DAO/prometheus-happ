# PROMETHEUS Gate 1 — Cryptographic Model Binding & Exact-DNA Synthetic Conductor Audit

Date: 2026-10-21
Status: **TEST-ONLY / SECURITY HARDENING CANDIDATE / DRAFT**
Source: PR #43, stacked on lab PR #39. Tracking issue: #41. No production DNA or Canon release changes.

## Exact artifact identity

- New isolated synthetic DNA SHA-256: `8b2f9e2321a580158a1c7ddebb18144b1b3ef8f7f65aeebbb6e75383083cfe9c`.
- Integrity WASM SHA-256: `e94e884d95bd8e450068eadb91418533f5031809bbe0be182e03f02182c71f2f`.
- Coordinator WASM SHA-256: `d54ed88928da29c86da23c4cee6021ee0e3f8e885a2e5d771f4dadeb9c7136df`.
- Original prior-lab DNA SHA-256: `8f60c6b9e9fe9d3722720376a5aa30ca49720268a2a8fdbd036f35f61aba6efc`. Original evidence is preserved; new DNA is distinct.

## Verified logic and build

- Native HDK 0.6.1 / HDI 0.7.1 host tests: **12 passed / 0 failed**, including rehash of actual synthetic model serialisation and deliberate baked-model modification with unchanged claimed digest. A malformed baked model is rejected.
- Independently verified GitHub Actions [run 39645963245](https://github.com/Permaculture-DAO/prometheus-happ/actions/runs/39645963245): kernel parity PASS; host integrity unit test PASS; integrity WASM compilation PASS; coordinator WASM compilation PASS.
- Local `cargo build --release --target wasm32-unknown-unknown` on both standalone zomes succeeded. `hc dna pack` produced above isolated TEST DNA. No production branch mutation.

## Actual new in-process Holochain conductor result

Linux WSL isolated conductor harness installed *only* new packed TEST DNA in a separate app. On `2026-10-21`:
```sh
cd /home/giova/prometheus-mrf-gate1-conductor-20261021/conductor_tests
OPENSSL_NO_VENDOR=1 CARGO_TARGET_DIR=/home/giova/prometheus-mrf-milestone-20261019/conductor_tests/target \
  timeout 360s cargo test --offline --locked --test conductor -- --nocapture --test-threads=1
```
Observed output (one Tokio test with eight assertions, NOT eight separate multi-peer tests):
```text
running 1 test
test synthetic_conductor_valid_and_adversarial_gates ... GATE_1_TEST_VALID=PASS
GATE_2_OUTPUT_FORGERY_REJECTED=PASS
GATE_3_REAL_REJECTED=PASS
GATE_4_MODEL_SUBSTITUTION_REJECTED=PASS
GATE_5_FINANCIAL_PROMOTION_REJECTED=PASS
GATE_6_MUTATION_REJECTED=PASS
GATE_7_DELETION_REJECTED=PASS
GATE_8_LINK_REJECTED=PASS
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 149.27s
```
Total call including host compilation took about 286 seconds.

**Test assertion limitation:** the seven negative scenarios assert that `call_fallible` returns an error. These prove that the operations do not succeed in the tested conductor, **not yet that every error is positively identified as an integrity-callback rejection**. A follow-up reason-matching test is required to exclude unrelated failures (bad API entrypoint, dependency, transport or argument shape) before full op-surface security claims. The code itself has explicit reject branches for StoreEntry, StoreRecord, RegisterUpdate, RegisterDelete, and link operations.

## Remaining NO-GO

- Error-cause-confirmed conductor negative tests; Holochain agent activity and other op variants; remote multi-peer DHT validation, partition/replay/sync tests; absence of unknown dependency bypass; producer/custodian approval; supply-chain signing and whole-DNA reproducibility.
- SHA-256 ties the embedded toy forest's serialised bytes to an approved constant; model hash binding still does **not** verify the scientific validity, ecological causality, source provenance, sensor truth or independent oracle governance.
- No REAL_DATA_PERSISTENCE_GATE_CLOSED change. No PRU/RAP/TRBK, capital, legal, insurance, certification or live rollout authority.

Disposition: **CONDITIONAL-GO** for the next isolated adversarial lab increment; **NO-GO** for real or production use.


## Diagnostic replay — rejection reason verified (2026-10-21)

After the first eight-scenario pass, the same in-process TEST DNA `8b2f9e2321a580158a1c7ddebb18144b1b3ef8f7f65aeebbb6e75383083cfe9c` was re-run with stronger tests. For every negative action, the test now inspects the `call_fallible` error representation and requires the specific expected integrity callback message; returning a generic error (e.g. function missing) no longer suffices.

Expected error markers, each verified in the executed harness:
- Output forgery → `synthetic MRF result mismatch`
- REAL submission → `REAL_DATA_PERSISTENCE_GATE_CLOSED`
- Invalid declared model hash → `model artifact hash`
- Financial promotion → `model-conformity authority`
- Update/delete attempts → `append-only`
- Unauthorized link → `do not support links`

**Actual execution output:** `GATE_1` to `GATE_8` all PASS; `test result: ok. 1 passed; 0 failed`; **101.92 seconds test runtime** (243.15 seconds including rebuild/replay invocation). This supersedes the earlier caveat that errors were not attributable to a specific callback, for these seven rejected scenarios on this DNA. It does not establish all possible FlatOp variants, multi-peer replication or absence of other bypasses.

**Important:** the original laboratory DNA, source record, production runtime and REAL gate remain unchanged. The strengthened test source is included in draft PR #43.
