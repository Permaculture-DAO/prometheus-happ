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


## Environment blocker on isolated peer-discovery replay — 2026-10-21

A second `peer_info_smoke.rs` run progressed past compilation, launched the test, then **FAILED** (one failed test; 111.57s test runtime) *before* completing the peer exchange:

```text
B: ConductorError(StateMutationError(DatabaseError(
  DbConnectionPoolError(Error(Some(
    "unable to open database file: /tmp/holochain-test-environments.../databases/wasm/wasm"
  )))
)))
test result: FAILED. 0 passed; 1 failed
```

This supports an environment/database-path initialization failure on the second in-process conductor, **not** a verified cryptographic or network consensus failure. Immediately afterward, basic WSL `df` commands failed to start, returning `getpwnam(giova) failed 5`, `getpwuid(1000) failed 5` and `fopen(/etc/default/locale) failed 5`, reinforcing an environment-level blocker. Do NOT silently reset WSL, delete databases or touch production services to force test success.

**Final network gate assessment:** initial combined harness printed `PEER_INFO_EXCHANGE=PASS` but did not finish its full test; the isolated smoke test ended FAILED before that check due database initialization. Thus independent two-conductor validation and DHT replication both remain **NOT VERIFIED / HOLD**. Run safe independent host filesystem/WSL health diagnostics, use an isolated writable temporary directory, preserve logs and determine root cause before rerunning. No claim of live site evidence, external independent oracles or scientific certification.


## Post-recovery gate reconciliation — 2026-10-21

Following confirmed host disk-pressure mitigation, a **separate, completed** `peer_info_smoke` test (same TEST DNA `8f32a4bf0ca75f04649671cc6d5b73cecebdb0bd36a51fb58700f3837717abca`) returned `TWO_CONDUCTORS_PEER_INFO_DISCOVERY=PASS`, **1 passed, 0 failed**, 75.23s, exit code 0. This supersedes the old environment-blocked discovery result for peer-info only.

A fresh replay of the compiled full two-conductor application harness with a Windows disk guard (C >=4 GiB), an Ubuntu `timeout 230s`, and source calls executed on two isolated conductors printed `PEER_INFO_EXCHANGE=PASS`, then did **not** complete the local synthetic receipt calls before timeout; the Windows wrapper exited 1 at ~242.09s. Free disk stayed ~4.235 GiB C / 0.631 GiB D. This outcome is **TIMEOUT / NOT PASS**, and must not be misreported as distributed inference validation, DHT replication or model rejection under a second peer.

The `two_conductors.rs` source now includes stage markers `STAGE_A_CREATE_BEGIN`, `STAGE_B_CREATE_BEGIN`, `STAGE_B_REAL_REJECTION_BEGIN` and per-call bounded timeouts. **This instrumented version has been committed but not executed** because of constrained host disk. Thus it is an unverified diagnostic change, not new evidence. See [Gate 5 cross-peer test contract](GATE5_DHT_REPLICATION_TEST_CONTRACT_2026-10-21.md) and [host recovery evidence](HOST_WSL_RECOVERY_EVIDENCE_2026-10-21.md).

The earlier single-conductor 7/7 PASS is unchanged. All real environmental/scientific/financial gates remain closed.


## Instrumented two-conductor replay — 2026-10-22

The previously committed per-stage diagnostic `two_conductors.rs` was actually synchronized to Federica's **isolated** WSL laboratory and executed against exact TEST DNA SHA-256 `8f32a4bf0ca75f04649671cc6d5b73cecebdb0bd36a51fb58700f3837717abca`.

Actual observed test output:
```text
PEER_INFO_EXCHANGE=PASS
STAGE_A_CREATE_BEGIN
STAGE_A_CREATE_PASS
STAGE_B_CREATE_BEGIN
```
The outer `timeout 280s cargo test --offline --locked --test two_conductors -- --nocapture --test-threads=1` process then returned **exit 1 at approximately 291.59 seconds** including WSL startup, without `STAGE_B_CREATE_PASS` or `SECOND_CONDUCTOR_REAL_REJECTION=PASS`.

**Disposition:** discovered where the test hangs: **the second coordinator receipt creation stage B**, after valid receipt creation succeeded at A; NO proof yet that B independently creates, that an attempted REAL on B is rejected, or that any A-authored DHT receipt is remotely retrievable on B. The 75-second Tokio wrapper did not produce an explicit stage-specific timeout before the outer process stopped, so do not assume the timer itself is effective under a blocked coordinator/conductor future. This observation is not proof of a defect in the toy numeric MRF nor of a specific network failure.

Host volume C had ~8.991 GiB free during the run; D ~0.634 GiB. No host data or production DNA changes. Follow-up targeted `conductor_b_only.rs` isolates B's creation with conductor A initialized but deliberately idle; **its result must be recorded separately** before interpreting whether sequential writes are causal.

**Remaining Gate-5 review:** distinguish entry author-local success, host runtime reentrancy/blocking, Holochain DHT authority integration, secondary agent validation and remote fetch. No REAL persistence, Canon promotion, oracle authenticity or capital authority.
