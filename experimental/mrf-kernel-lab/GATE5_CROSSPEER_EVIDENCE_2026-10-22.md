# PROMETHEUS MRF Gate 5 — exact-DNA network lookup evidence
**Date:** 2026-10-22 · **Status:** NO-GO for distributed receipt proof / TEST-only engineering remains admissible.  
**Branch:** `experiment/mrf-gate5-crosspeer-read-20261022` · Parent draft [PR #48](https://github.com/Permaculture-DAO/prometheus-happ/pull/48) · [Gate 5 PR #50](https://github.com/Permaculture-DAO/prometheus-happ/pull/50).

## Source and artifact identity
- Frozen synthetic model PMRF-BIN-v1 SHA-256 `e074715a26129c8f00e828774eadc71ef33f94a620c0dcd0e00216b6156bda5f`.
- Gate 5 coordinator WASM SHA-256 `0e4b6e2531536e7b33f41f3806c284d1569cc805dcf870cf50fa4eff821a59b1`.
- Retained Gate 4 integrity WASM SHA-256 `6cefdc096c665a7662fc48351800d94be31d5ad8d4450efce03cd4c9bdf61c93`.
- **Exact NEW packed TEST DNA** SHA-256 `1d20d26ec6d03af81e3cc7f85906615c4894c65bc843b6f949f969afb851d009`.
- Coordinator host `cargo check --offline` **PASS** after explicit error mapping; WASM `cargo build --offline --release --target wasm32-unknown-unknown` **PASS**.
- Independent GitHub Actions `mrf-gate5-readonly-coordinator` coordinator WASM compile job **PASS** on run [39722725638](https://github.com/Permaculture-DAO/prometheus-happ/actions/runs/39722725638). Different compiler/transitive build hash should not be assumed byte-identical without independent frozen lockfile proof.

## Runtime test 1: cross-conductor network get — FAILED
Environment: isolated Ubuntu WSL lab `/home/giova/prometheus-mrf-gate5-20261022`, two separately initialized TEST-only SweetConductors, B does not author a receipt. Exact new DNA above.

```sh
OPENSSL_NO_VENDOR=1 timeout 410s cargo test --offline --locked \
  --test crosspeer_read -- --nocapture --test-threads=1
```
Observed:
```text
CROSSPEER_STAGE_INIT_BEGIN
CROSSPEER_PEER_INFO_PASS
CROSSPEER_A_CREATE_PASS
CROSSPEER_B_NETWORK_GET_ATTEMPT_1
CROSSPEER_B_GET_TIMEOUT: Elapsed(())
test result: FAILED. 0 passed; 1 failed; finished in 209.83s
```
The containing process returned exit 1 after ~259.58s including compilation and WSL startup. **Interpretation:** creation on A succeeded and B attempted HDK `get(original_action_hash, GetOptions::network())` but **did not return before the explicit 40-second deadline**; typed content equality was NOT established, and there is **NO cross-peer/DHT replication PASS**. It does not by itself prove whether root cause lies in network transport, peer DHT integration, a blocked host zome call or timing constraints. No fallback to unverifiable 'success' allowed.

## Runtime test 2: author-local endpoint sanity — SEPARATE GATE
**PASS:** a completed author-local `local_read_sanity.rs` test used the **same Gate-5 DNA SHA-256 `1d20d26ec6d03af81e3cc7f85906615c4894c65bc843b6f949f969afb851d009`**. Output:
```text
LOCAL_A_CREATE_BEGIN
LOCAL_A_CREATE_PASS
LOCAL_A_GET_BEGIN
LOCAL_A_GET_EXACT_ACTION_PASS
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 73.17s
```
Windows wrapper exit 0 after ~178.45s, including compilation/startup. The endpoint correctly fetched the author's original action and checked hash, TEST class, model hash, numeric vector and financial-admission flag. **This supports the local coordinator/read/serialization path only; it does NOT prove any cross-peer network read.**

## Existing baseline not promoted
Previous Gate 4 single-conductor 7/7 bounded PASS; peer-info two-conductor discovery PASS; B-only receipt creation PASS; reversed B→A sequential writes PASS; A→B sequential writes timed out. These verify separate hypotheses and do not establish DHT replication.

## Next architecture decision / open blockers
1. Distinguish endpoint functionality (author-local get) from remote network access. Pin exact rejection reason / type on failed callbacks.
2. Investigate Holochain 0.6.1 SweetConductor inter-peer DHT publication/authority integration, discovery metadata, transport and default GetOptions network timeout; benchmark on independent test runner and record time-to-convergence.
3. After read succeeds, verify original action, payload, author identity and hashes on B; test incorrect hash, forged ops, partitions, disappearance, replay and recovery; prohibit inference of real operator/custody evidence.
4. CI whole-DNA rebuild, pinned lockfile and responsible steward approval prior to any production promotion.

**Hard invariants:** `REAL_DATA_PERSISTENCE_GATE_CLOSED`; no signed Canon or live runtime modifications; scientific proof, oracle independence, ecological improvement, financing, PRU/RAP/TRBK rights all remain unestablished.

## Evidence-constrained conclusion
**GO:** standalone Rust/HDK 0.6.1 check, Gate-5 coordinator WASM compile locally and independently, packed synthetic DNA, author-local exact-hash record lookup. **HOLD/NO-GO:** B remote DHT fetch (40-second timeout, 0/1 test), multi-peer authority validation, gossip/partition/rejoin, empirical sensors and finance. The combination of author-local PASS and B-remote timeout is consistent with a problem in the cross-peer path or its timing, but is **not a root-cause proof**. Keep REAL closed.


## Independent CI and workstation safety closure — 2026-10-22

**Verified:** All five GitHub Actions workflows associated with latest hApp PR #50 head completed **successfully**: `happ-ci`, `gateway-ci`, `secret-scan`, `mrf-kernel-synthetic-parity` and `mrf-gate5-readonly-coordinator`. These independent checks **compile and test selected components**, but they DO NOT execute or pass the failing multi-peer DHT scenario.

**Host capacity guard:** free Windows C dropped from ~9.24 GiB before the sequence to ~3.905 GiB after WASM/host builds; D last measured ~1.738 GiB. The packed TEST DNA and separate coordinator WASM copy were present, and `certutil SHA256` verified the coordinator copy hash as `0e4b6e2531536e7b33f41f3806c284d1569cc805dcf870cf50fa4eff821a59b1`.

After confirming no `cargo`/`rustc` processes, no reparse point and preservation of `Cargo.lock`, source and packed TEST DNA, only the **newly generated** `zome_coordinator_lab_gate5/target` cache (approx. 0.64 GiB) was removed on Federica. C then had ~4.342 GiB free. No application databases, source branches, signed Canon, VHDX, active runtime, keys or project archives were deleted. No further heavy conductor attempts were launched.

**Final disposition:** local synthetic model receipt Get endpoint GO; B cross-peer DHT Get NO-GO (failed, timeout 40s); production and scientific/financial gates unchanged. Engineering follow-up: [Issue #51](https://github.com/Permaculture-DAO/prometheus-happ/issues/51).
