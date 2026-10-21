# PROMETHEUS MRF Gate 1/2 — Operative Synthetic Evidence Record

**Scope:** `experiment/mrf-gate1-flatop-gate2-registry-20261021` on top of draft PR #45 → PR #39. No merge or production deployment.  
**Date:** 2026-10-21. **Claim ceiling:** `TEST_MODEL_CONFORMITY_AND_POLICY_ONLY`, no ecological certification, capital, PRU/RAP/TRBK, REAL evidence or independent regulatory/insurer authority.

## Reconciled local WSL evidence
1. **HDI 0.7.1 FlatOp integration:** 10/10 host integrity-zome tests PASS after adding StoreRecord/StoreEntry conformity checks and fail-closed update/delete/link branches.
2. **Signed test oracle registry:** 9/9 unit tests PASS. Ed25519 against test-provisioned public-key registry; keyed operator/control groups; activation and revocation; evaluation time; distinct sample and data hashes; nonce replay within batch; disagreement. Prior test fixture failure was a legitimate Ed25519 detection on a tampered sample label; fixed by *re-signing the deliberately duplicated sample* so the test actually reached the duplicate-detection policy.
3. **Ecological anti-Goodhart sentinels:** 8/8 Rust tests PASS. Non-compensatory soil/water/biodiversity/yield/external input checks, incomplete baseline/comparator/method drift/replay. **Never a scientific finding**; all-positive fixture returns only `ReviewCandidate`.
4. **WASM integrity build:** PASS, `prometheus_mrf_integrity_lab.wasm` SHA-256 `c81504f2a78d9ffcd3975cc137b7ac0d1790a173d36ae5008eb0615feeba7b73`.
5. **WASM coordinator build:** PASS, `prometheus_mrf_coordinator_lab.wasm` SHA-256 `b50fba0d80b8d9b26b315ec7e380ded6acb183e861329b3081a6f747c191505d`.
6. **NEW isolated TEST DNA pack:** SHA-256 `a444187b2c1e4eb4307c81adf72f545dd78e0ff27e8d2f64aca8b0952e9db62b`. This is not an externally attested or signed production DNA.
7. **Live in-process conductor:** exact NEW DNA, **one sweettest with 8/8 gate scenarios PASS** in 123.10 seconds. Outputs:
```
GATE_1_TEST_VALID=PASS
GATE_2_OUTPUT_FORGERY_REJECTED=PASS
GATE_3_REAL_REJECTED=PASS
GATE_4_MODEL_SUBSTITUTION_REJECTED=PASS
GATE_5_FINANCIAL_PROMOTION_REJECTED=PASS
GATE_6_UPDATE_REJECTED=PASS
GATE_7_DELETE_REJECTED=PASS
GATE_8_LINK_REJECTED=PASS
test result: ok. 1 passed; 0 failed; finished in 123.10s
```
Test path in WSL: `/home/giova/prometheus-mrf-flatop-gate3-20261021/conductor_tests`; `OPENSSL_NO_VENDOR=1 cargo test --offline --locked --test conductor -- --nocapture --test-threads=1`. Includes a compile step outside the 123.10-second test runtime.

## Distinct second gate
Two separate-conductor consistency scenario `peer_consistency.rs`: **not yet verified in this initial record**. Its compilation/test result must be captured explicitly; a passing one-conductor test is not peer consistency evidence.

## Residual risks / no-go
- Full Holochain op validation in a multi-zome environment cannot be inferred from deliberately narrow, strict single-type TEST DNA. Source activity ops may lack entry payload.
- Operator/control group independence is presently an assumed synthetic registry label; no corporate-control attestation, signed policy release, trustworthy key revocation or physical source trust has been proven.
- Time/revocation/replay state is test-local and not persistently coordinated across disconnected hosts or across restarts; signature alone has no trusted timestamp.
- Multi-peer partitions, offline recovery, malicious source chains, peer revocation, operator collusion, sensor provenance and experimental causal MRV not cleared.
- Ecological sentinel policy thresholds are illustrative, not approved science. No site data.
- Production runtime signed release and `REAL_DATA_PERSISTENCE_GATE_CLOSED` remain untouched. GitHub branches remain draft.

**Gate disposition:** CONDITIONAL GO for exact-DNA lab model-conformity / negative cases only; HOLD on cross-host and field/finance admission.


## Verified multi-conductor DHT consistency — 2026-10-21

The provisional *peer-consistency PENDING* note above is superseded **only for this precise TEST scenario**.

**WSL test output** (`conductor_tests/tests/peer_consistency.rs`):
```
test separate_conductors_replicate_only_valid_test_receipts ... GATE_TWO_CONDUCTORS_STARTED=PASS
GATE_TWO_CONDUCTORS_CONSISTENCY=PASS
GATE_TWO_CONDUCTORS_FORGERY_REJECTED=PASS
ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 233.95s
```

**How executed:** two separately instantiated `SweetConductor` instances with distinct agent keys and per-conductor test environments in **one localhost/WSL test process**, `SweetConductorBatch::from_standard_config_rendezvous(2)`. Installed the same exact TEST DNA SHA-256 `a444187b2c1e4eb4307c81adf72f545dd78e0ff27e8d2f64aca8b0952e9db62b`; awaited initial consistency; each agent submitted a conforming synthetic receipt; `await_consistency` returned success; forged receipt was rejected on conductor B. End-to-end tool runtime including build ~361.36s; test-only runtime 233.95s.

**Supported:** local two-conductor TEST DHT consistency and one forged receipt rejected. **NOT supported:** physically independent hosts, production deployment, public network, partitions, censorship, hostile peer quorum, real source data, ecological verification, independent oracle custody, financial/token authority. No REAL gate changed.
