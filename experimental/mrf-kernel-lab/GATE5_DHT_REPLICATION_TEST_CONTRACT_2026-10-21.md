# PRM-MRF-G5-DHT-001 — Cross-peer validation test contract (PROPOSED)

**Status:** specification only; no implementation or test PASS claim.  
**Canonical precedence:** PROMETHEUS controlled candidate in [canon PR #29](https://github.com/Permaculture-DAO/prometheus-canon/pull/29) remains unratified. Runtime signed DNA is not modified.  
**Engineering dependencies:** [hApp PR #48](https://github.com/Permaculture-DAO/prometheus-happ/pull/48), [Issue #41](https://github.com/Permaculture-DAO/prometheus-happ/issues/41), [HOST-WSL-01 #49](https://github.com/Permaculture-DAO/prometheus-happ/issues/49).  
**Scope:** TEST-only, integer toy MRF. `REAL_DATA_PERSISTENCE_GATE_CLOSED` is invariant.

## Goal and falsifiable hypotheses
H1: A different Holochain agent B can independently retrieve an immutable, well-formed MRF receipt written by agent A after integration in the **same** test DNA space, with deterministic data decoded and model SHA-256 checked. H2: B's validation path rejects a forged receipt on incoming DHT ops even when the forged author attempted to bypass coordinator controls. H3: a revoked, amended or deleted original record cannot be misrepresented as historical absence. H4: after intentional connectivity disruption, results are either convergent in a bounded recovery window or recorded UNKNOWN, never silently certified.

Any unverified requirement is NOT DEMONSTRATED; failure to converge within timeout is an explicit incomplete or FAIL outcome, never inferred success.

## Ordered test stages
1. **Preflight:** two independently initialized conductor state stores/agent keys, pinned HDK 0.6.1/HDI 0.7.1, exact TEST DNA digest, no production app context. Host physical free space recorded before and after. Strongly prefer isolated ephemeral CI machine to resource-constrained Federica (C <5 GiB and D <1 GiB as observed on 2026-10-21).
2. **Bounded discovery:** prove that both conductor instances have exchanged peer info, with completed test exit status. Already demonstrated in a narrow recovered-Windows/Ubuntu test, but not DHT.
3. **Author receipt:** A creates a valid `MrfSyntheticReceipt` with `evidence_class=TEST`, synthetically generated vector, model digest and `financial_admission=false`. Persist the returned *ActionHash* and resulting *EntryHash* to the test log. Never include private signing keys.
4. **Independent DHT fetch:** B invokes a **new explicitly versioned TEST-only coordinator endpoint** to retrieve A's action via Holochain `get(action_hash)` with supported `GetOptions`; the result MUST include the same action hash and original signed data. Record typed decode and hash equality; distinguish B's own authored entries from remote retrieval.
5. **DHT validation provenance:** log B's observation of stored DHT ops and validation outcome in supported test hooks. A successful `get` alone does not prove every DHT authority validated the op. Specify authority-level assertions separately.
6. **Negative injection:** inject unauthorized update/delete, REAL-class evidence, wrong actual-model digest and forged inference; demand typed `Invalid` errors with the expected rejection code, not just `is_err()`. Verify the original valid ActionHash remains retrievable.
7. **Transport adversaries:** simulated peer disconnect or network partition, delayed ops, rejoin, duplicate/replay, corrupted payload, missing dependency. Record per-peer observations and explicit time-to-convergence; never treat network quorum as measurement truth.
8. **Reproducibility:** source commit, Cargo.lock, Rust toolchain, WASM sha256s, packaged DNA sha256, sealed test fixtures and complete run output archived in draft PR. An independent CI runner must reproduce the essential results before promotion.

## Minimal PASS requirements
- Full test completes within a predeclared bounded runtime; no stale process is interpreted as success.
- A and B have separate agent keys and local state; remote B does not author or locally fabricate A's ActionHash.
- B returns the identical original A action and accepted receipt bytes; model sha256 and TEST boundaries hold.
- A forged remote record is conclusively rejected on its intended HDI operation, with error provenance recorded.
- Failed mutation does not erase original provenance.
- Partition/rejoin result reproducible or UNKNOWN with residual risk documented, no hidden optimistic assumptions.
- At least one independent runner confirms the result on an exact reconstructed synthetic DNA.

## Scientific and regulatory NON-GO
Even passing all tests does NOT establish physical provenance, instrument calibration, owner independence, scientific causality, anti-Goodhart or financial/certification rights. REAL and PRU/RAP/TRBK issuance remain separate steward and legal gates.
