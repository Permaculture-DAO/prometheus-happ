# MRF Gate 4: exact DNA append-only conductor evidence — 2026-10-21

## Controlled scope
- Draft [PR #48](https://github.com/Permaculture-DAO/prometheus-happ/pull/48), child of draft PR #47; synthetic research only.
- HDK 0.6.1 / HDI 0.7.1, frozen two-feature toy model; **no changes to production or signed Canon**.
- The source remains a controlled candidate; no REAL data, certified ecological indicators, economic rights, token actions or capital decisions permitted.

## Exact TEST DNA and compiled zomes
- Packed TEST-only DNA SHA-256 `8f32a4bf0ca75f04649671cc6d5b73cecebdb0bd36a51fb58700f3837717abca`.
- WASM integrity-zome SHA-256 `6cefdc096c665a7662fc48351800d94be31d5ad8d4450efce03cd4c9bdf61c93`.
- WASM coordinator-zome SHA-256 `f3957d995c57ab662d08a0b4bfc062acbd32a38515633f07157a5be70cc09027`.
- Both zomes compiled to `wasm32-unknown-unknown` successfully on Federica WSL. Host HDI unit tests 10/10 pass. Independent GitHub Actions parity and HDI host test jobs PASS at 2026-10-21.

## Single in-process conductor test: 7/7 expected outcomes
Isolated WSL test directory `/home/giova/prometheus-mrf-gate4-20261021/conductor_tests`. Command:
```sh
OPENSSL_NO_VENDOR=1 timeout 300s cargo test --offline --locked --test conductor -- --nocapture --test-threads=1
```

Executed **one Tokio test containing seven assertions**, with output:
```text
GATE_1_TEST_VALID=PASS
GATE_2_OUTPUT_FORGERY_REJECTED=PASS
GATE_3_REAL_REJECTED=PASS
GATE_4_MODEL_SUBSTITUTION_REJECTED=PASS
GATE_5_FINANCIAL_PROMOTION_REJECTED=PASS
GATE_6_UPDATE_REJECTED=PASS
GATE_7_DELETE_REJECTED=PASS
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 124.85s
```
Total wall time including Rust test compilation around 264s. The update/delete test uses real coordinator hApp host calls against this TEST DNA; an error was returned for each operation.

**Important limit:** the test currently asserts `call_fallible(...).is_err()`, not the exact HDI error message. Therefore this evidence establishes that both attempts failed through the conductor path, but **does not independently pinpoint the failure cause**. An additional reason-assertion test and cross-peer DHT corroboration remain necessary.

## What is NOT demonstrated by the 7-scenario PASS
- Enforcement under adversarially authored remote DHT ops / multiple independently owned agents / partitions.
- Error-reason specificity, replay behavior, immutability of original entry under all Holochain op variants and dependency states.
- Real-world ecological fact, true oracle independence, instrumentation calibration or insurance/financial risk truth.
- Governance approval, release signing, legal authority, REAL_DATA_PERSISTENCE_GATE_CLOSED opening, TRBK/PRU/RAP rights.
- Whole DNA byte-identical build across independent CI runners.

## Next required gate
Run separate two-conductor exchange and local/remote read checks; add exact rejection-reason assertions; explicitly test RegisterUpdate / RegisterDelete op variants and network partition/rejoin; preserve original action and show it is retrievable after failed mutation. Independently review new DNA artifact hashes and source governance before any controlled admission.


## Two-conductor distributed gate — partial / NOT PASS

A separate `two_conductors.rs` suite using two independently initialized TEST-only SweetConductors and the exact Gate-4 packed DNA completed `SweetConductor::exchange_peer_info([&a,&b])` and printed `PEER_INFO_EXCHANGE=PASS`. The following dual-conductor write/validation calls **did not finish with a passing test result** within the bounded run (outer process returned exit 1 after 446.61 s including compilation). **Do not claim two-conductor model validation or DHT replication.**

To isolate the success from the incomplete stage, a standalone `peer_info_smoke.rs` asserts just the peer exchange and prints its own final test result. This is a separate outcome; its execution status must be recorded before marking even this narrower smoke PASS.

Remaining required: explicit cross-peer get/validation of a known action, propagation convergence, peer partition, remote forged op rejection, original record retained after failed mutations, revocation and hash policy under distinct conductors. One-conductor 7/7 gate remains valid and unchanged.
