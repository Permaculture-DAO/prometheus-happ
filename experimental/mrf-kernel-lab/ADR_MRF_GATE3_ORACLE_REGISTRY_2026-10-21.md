# ADR-MRF-012 — Synthetic cryptographic oracle identity registry
Date: 2026-10-21. Source lineage: controlled candidate PRM-SRC-MRF-ZOME-EDGE-001 (not admitted).
Scope: PR #45 follow-on, `experimental/mrf-kernel-lab`, **TEST ONLY**.

## Local test evidence
The `oracle_registry_crypto.rs` regression harness uses pinned ed25519-dalek 2.2.0, static TEST keys and deterministic length-framed domain-separated messages.
Executed: `cargo test --offline --test oracle_registry_crypto` (on Federica WSL): **8/8 passed, 0 failed**.

Assertions: three distinct registered operators produce candidate agreement; same operator controlling multiple keys rejected; tampered measurement rejected; revoked signer rejected; expired and future-dated attestations rejected; duplicate sample rejected; differing but correctly signed measurements flagged disagreement; unknown signer rejected.

## Decision boundary
- Signatures bind a claim's fields, not truth of physical observations.
- The registered operator ID and revocation list are hypothetical trusted inputs in this harness, **not independently established real-world company-control evidence**.
- This code is a standalone Rust test, **not** a Holochain integrity callback, deployed service or distributed consensus mechanism.
- The sample uniqueness and seen-sequence policy will need persistent state with proper distributed race semantics; mutable global seen-sets must never be assumed available in a pure Holochain validation callback.
- Staleness policy must use a deterministic, authenticated, allowed reference from the Holochain operation or signed epoch; do not call wall-clock time or network endpoints from integrity validation.
- Revocation must use independently attested, versioned and causally available registry state, with clear treatment of retroactive invalidation.
- Operator independence requires verified control-group identity; distinct keys or nominal company labels do not prove independent custody or measurement.
- No REAL, scientific certification, token, rights, underwriting or capital implications.

## Next required tests
1. Independently review Holochain HDI 0.7.1 FlatOp layout, including StoreRecord, RegisterUpdate, RegisterDelete and dependencies; implement deny/explicit policy only after typed review.
2. Two or more real conductor instances / agents with malformed actions, partitions, stale hashes and deterministic result replay; record exact updated DNA hash.
3. Registry update/revocation propagation, signature replay and multiple devices sharing control; benchmark against a simpler signed-database baseline.
4. Ecological anti-Goodhart constraints, calibration audit and preregistered independent site comparators.
5. Steward approval for source admission and any production change. Do not merge or deploy by implication.
