# ADR-MRF-013 — Append-only synthetic receipts and explicit HDI FlatOp coverage

**Date:** 2026-10-21  
**Source dependency:** `PRM-SRC-MRF-ZOME-EDGE-001` (candidate) → architecture `PRM-ARCH-DET-002` (controlled) → controlling Canon.  
**Status:** Proposed / experimental / TEST-only. No signed production DNA change, no REAL admission, no scientific or capital authority.  
**PR ancestry:** Gate 4 branch derives from draft oracle-registry PR #47 (which derives from #45 → #39).

## HDI 0.7.1 checked against actual installed source
Inspected:
- `hdi-0.7.1/src/flat_op.rs`
- `hdi-0.7.1/src/flat_op/flat_op_record.rs`
- `hdi-0.7.1/src/flat_op/flat_op_entry.rs`

These define `StoreRecord(OpRecord<ET,LT>)`, `StoreEntry(OpEntry<ET>)`, `RegisterUpdate(OpUpdate<ET>)`, `RegisterDelete(OpDelete)`, `RegisterCreateLink`, `RegisterDeleteLink`, and `RegisterAgentActivity` with **different payload availability**. An `OpDelete` does not expose the original app entry body; blanket field checks on Delete cannot reconstruct earlier evidence.

## Policy of this single-entry-type laboratory DNA
- A `StoreRecord::CreateEntry` or `StoreEntry::CreateEntry` containing an application receipt is eligible for the **same** SHA-256 baked-model binding and TEST-only body validation.
- `StoreRecord::UpdateEntry`, `StoreEntry::UpdateEntry`, `RegisterUpdate::Entry`, and their private variants: **Invalid** unconditionally; history is append-only.
- `StoreRecord::DeleteEntry` and `RegisterDelete`: **Invalid**; deleting TEST receipts is forbidden in this *one-type experimental DNA*.
- `RegisterCreateLink`, `RegisterDeleteLink`, `StoreRecord::CreateLink`, `StoreRecord::DeleteLink`: **Invalid**. Future corrections and supersession must receive a designed, separately reviewed immutable provenance type—not ad hoc link rewrites.
- Non-app Holochain system operations (e.g. agent activity, chain system data and capabilities) remain governed by the normal host rules; this policy must **not** be copied indiscriminately into a production multi-type DNA.
- No code-path special-cases the phrase "REAL": REAL is categorically rejected by receipt validation.

## Verified vs open
- **Verified locally:** host-side unit tests 10/10 passed on Federica WSL after compiling explicit FlatOp matches. WASM integrity-zome + coordinator compilation completed, and an isolated TEST-only DNA has been packaged.
- **Exact Gate-4 DNA SHA-256:** `8f32a4bf0ca75f04649671cc6d5b73cecebdb0bd36a51fb58700f3837717abca`; integrity-zome WASM SHA-256 `6cefdc096c665a7662fc48351800d94be31d5ad8d4450efce03cd4c9bdf61c93`; coordinator WASM SHA-256 `f3957d995c57ab662d08a0b4bfc062acbd32a38515633f07157a5be70cc09027`.
- **Still separate runtime gate at authoring:** test seven scenario outcomes on exact DNA (five prior plus update/delete), two-conductor exchange and cross-peer retrieval; neither can be inferred from compilation.
- **Risks:** complex op-type policy may reject Holochain-required structures, raw deletes with unavailable original body, bad local actions with deceptive error codes, backward compatibility, missing dependency behavior, partitions. Every negative test must assert the intended rejection reason where available.

## Governance boundary
Do not merge this draft into the signed hApp, replace the deployed `hearth` DNA, lift `REAL_DATA_PERSISTENCE_GATE_CLOSED`, publish ecological metrics, issue rights or release capital. Preserve physical evidence, claims review, risk methods Aeterna/Ravel, PRU/RAP/TRBK separation and competent steward approval.
