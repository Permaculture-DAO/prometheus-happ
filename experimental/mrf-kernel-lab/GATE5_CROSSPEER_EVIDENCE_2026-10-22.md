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
A single-conductor author-local `local_read_sanity.rs` test was authored to create and then fetch the same action on A using the same new endpoint and exact DNA. It is **not the cross-peer proof**; result must be recorded only when the test exits.

## Existing baseline not promoted
Previous Gate 4 single-conductor 7/7 bounded PASS; peer-info two-conductor discovery PASS; B-only receipt creation PASS; reversed B→A sequential writes PASS; A→B sequential writes timed out. These verify separate hypotheses and do not establish DHT replication.

## Next architecture decision / open blockers
1. Distinguish endpoint functionality (author-local get) from remote network access. Pin exact rejection reason / type on failed callbacks.
2. Investigate Holochain 0.6.1 SweetConductor inter-peer DHT publication/authority integration, discovery metadata, transport and default GetOptions network timeout; benchmark on independent test runner and record time-to-convergence.
3. After read succeeds, verify original action, payload, author identity and hashes on B; test incorrect hash, forged ops, partitions, disappearance, replay and recovery; prohibit inference of real operator/custody evidence.
4. CI whole-DNA rebuild, pinned lockfile and responsible steward approval prior to any production promotion.

**Hard invariants:** `REAL_DATA_PERSISTENCE_GATE_CLOSED`; no signed Canon or live runtime modifications; scientific proof, oracle independence, ecological improvement, financing, PRU/RAP/TRBK rights all remain unestablished.
