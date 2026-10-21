# ADR-MRF-012 — Append-only TEST receipts and independent-oracle-key policy

**Date:** 2026-10-21 · **State:** PROPOSED / TEST / NON-CANONICAL  
**Upstream:** [MRF baseline PR #39](https://github.com/Permaculture-DAO/prometheus-happ/pull/39) → [SHA-bound model PR #45](https://github.com/Permaculture-DAO/prometheus-happ/pull/45) → this laboratory increment.  
**Issues:** [Gate #41](https://github.com/Permaculture-DAO/prometheus-happ/issues/41) and [Gate #42](https://github.com/Permaculture-DAO/prometheus-happ/issues/42).

## Evidence and authority boundaries
The reference Canon / PRM-ARCH-DET-002 mandates evidence-before-value, D verification and human/domain review. Validating MRF outputs is not ecological truth; independently authenticated signatures are not proof of independent measurements or operator control. No capital, risk transfer, REAL evidence admission, PRU, RAP or TRBK state is modified here.

## 1. Holochain HDI op matrix (single-type synthetic laboratory ONLY)

| Op family | Policy | Rationale / remaining proof |
|---|---|---|
| StoreRecord / CreateEntry and StoreEntry / CreateEntry | Recompute baked PMRF model SHA-256 and deterministic output; reject errors | Both action and entry DHT authorities need conformity checks |
| StoreRecord / UpdateEntry and StoreEntry / UpdateEntry | INVALID | Immutable receipts never accept amendments, even if new prediction is identical |
| RegisterUpdate / Entry and PrivateEntry | INVALID | A mutation must not bypass the entry authority |
| StoreRecord / DeleteEntry and RegisterDelete | INVALID | TEST receipts are append-only; do not silently erase provenance |
| RegisterAgentActivity / UpdateEntry, UpdatePrivateEntry, DeleteEntry | INVALID where identifiable as receipt; any delete invalid in this isolated single-type DNA | Source chain metadata has no entry body, therefore don't infer full model validation |
| StoreRecord / CreateLink and DeleteLink; RegisterCreateLink / RegisterDeleteLink; matching AgentActivity | INVALID | No link rights in this isolated DNA |
| Default Holochain DNA, agent-registration, init / unrelated metadata | VALID | Normal Holochain lifecycle must remain operational |

**Important portability caution:** current callback is intentionally strict for a one-entry synthetic DNA. A multi-zome production hApp must narrow op-level rejection to the correct entry/link type and establish original-action provenance for deletes. Do not copy this callback verbatim to production.

## 2. Experimental coordinator attack paths
The coordinator exposes **negative-test-only** `update_synthetic_receipt`, `delete_synthetic_receipt`, and `link_synthetic_receipts` endpoints. They are present solely to verify integrity callback rejection on the isolated synthetic DNA. They must not be deployed in the public runtime.

## 3. Oracle registry hardening
A TEST registry maps Ed25519 public keys → operator labels → control-group labels, activation and revocation instants. Synthetic independently signed payloads use versioned, domain-separated, length-delimited fields including site, method, sample, data hash, timestamp, expiry, nonce and value. The test-side evaluator checks:
- actual Ed25519 signature verification against registry public key;
- key registration, revocation and bounded freshness with externally supplied `now`;
- per-control-group uniqueness, per-sample and payload-hash duplication, per-key nonce replay *within the supplied batch*;
- method/site consistency, minimum independent-control group quorum and explicit disagreement.

A passing outcome is **CANDIDATE_AGREEMENT**, never Verified Indicator, scientific causality, actual operator independence, anti-Sybil certification, legal or financial admissibility.

## 4. Test/evidence status
- Local host HDI unit tests: **10/10 PASS** after FlatOp code change (2026-10-21).
- Signed TEST registry suite: **9/9 PASS** after corrected, deliberately re-signed duplicate vector (2026-10-21). First attempted vector changed a signed payload without re-signing; it properly failed signature validation; test fixture was amended to reach the duplication policy.
- New exact-DNA WASM compile and conductor update/delete/link tests: **PENDING results** at the time of this ADR's initial creation. Recompute new DNA hash and report PASS/FAIL separately.
- Multi-peer/independent-conductor DHT results: **NOT EXECUTED** by these tests; do not infer global consensus.
- Registered oracle control groups and clock source, revocation snapshots, external measurement/certification: **NOT DEMONSTRATED**.

## 5. Gate criteria and simplification
Do not promote beyond TEST before (1) exact DNA conductor rejects all mutation paths, (2) multi-agent DHT replication tests, (3) signed frozen oracle registry with anti-replay across restarts and revocation controls, (4) external source calibration and ecological independent MRV, (5) third-party review and governance. Benchmark against a signed conventional append-only database and reject needless complexity.
