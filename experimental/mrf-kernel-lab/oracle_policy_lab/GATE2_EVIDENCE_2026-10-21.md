# PROMETHEUS Gate 2 — Synthetic Oracle and Anti-Gaming Evidence

Date: 2026-10-21
Authority: RESEARCH / SHADOW / TEST ONLY
PR: [#44](https://github.com/Permaculture-DAO/prometheus-happ/pull/44); Issue: [#42](https://github.com/Permaculture-DAO/prometheus-happ/issues/42)
Parent: [synthetic MRF lab #39](https://github.com/Permaculture-DAO/prometheus-happ/pull/39).

## Executed evidence

[Independent GitHub Actions run 39645604603](https://github.com/Permaculture-DAO/prometheus-happ/actions/runs/39645604603): Rust oracle shadow test suite **13/13 PASS, 0 failed**. Actual Ed25519 signatures are verified for domain-separated canonical TEST claims. The 13 tests include positive 2-group/2-method agreement, same-control-group correlated signers, signature forgery, duplicate sample digest, freshness, revocation, absent calibration, contradiction, altered signed claim, rogue key and ecological degradation/missingness sentinels.

The test identities are deterministic artificial fixture keys. They are **not** field devices or trusted real-world identities.

## What the prototype can assert

- `Agreement`: policy-conformant agreement among at least 2 different *registered* operator groups and 2 different *registered* methods with valid signatures and distinct submitted sample digests.
- `Disagreement`: conflicting verified claims.
- `Unknown`: insufficient independent groups or methods (or missing ecological sentinel input).
- `RequiresReview`: policy violation, revoked/unknown key, freshness failure, missing calibration flag, duplicate signer/device/sample digest, tampered signature or ecological regression.

An `Agreement` is a **cryptographic statement about submitted bytes**; it is NOT MRV-admissible ecological proof or delegated institutional authority.

## Threat model and residual gaps

| Attack/failure | Current mitigation | Residual | Next test |
|---|---|---|---|
| Forged signature/altered vote | Ed25519 verify over domain-separated payload | key theft | key custody/revocation drill |
| Sybil: multiple keys, same control | trusted registry groups + distinct-method quorum | concealed common ownership | independent operator due diligence |
| One device, multiple keys | registered device uniqueness | physical cloned device IDs | device attestation and calibration cross-check |
| Same evidence reused | repeated sample hash rejected | attacker fabricates different hashes | field physical-origin cross-sampling |
| Replay | timestamp window and batch-local duplicate detection | replay across independent evaluation calls/persisted batches | signed replay ledger/policy epoch |
| Stale/revoked source | frozen-time window and revocation set | policy registry update governance | hashed registry snapshot/revocation proofs |
| False calibration | declared calibration flag required | flag may be fabricated | calibration certificate chain and site methods |
| Ecological outcome laundering | non-compensating degradation sentinels | missing units, counterfactual, uncertainty and physical QA | preregistered field/comparator protocol |
| Collusive independent labs | partial method/operator separation | unknown beneficial control and shared instrument vendor | adversarial independent review |
| DHT/mesh failure | none; research crate is offline | synchronization, partitions, transport and power unknown | multi-peer Holochain and Reticulum/edge test plan |

## Required next gates

1. Bind oracle registry/policy epoch, signer revocation and sample digest to versioned, signed manifests; verified by deterministic code, never an LLM.
2. Add persistent replay ledger and revocation history, test partition reconciliation and disputed independent sources.
3. Keep causal ecological attribution separate from model prediction; record baseline, control, units, calibration, missingness, uncertainty and independent reviewers.
4. Perform device/edge benchmark vs simpler signed-record and offline spreadsheet methods before adding more middleware.
5. Design, do not claim, Reticulum transport integration; Holochain DHT and Reticulum provide different security/consistency layers.

No REAL_DATA_PERSISTENCE_GATE_CLOSED change; no tokens, PRU value, RAP, financing, certification or production authorization. Gate 2 remains **SHADOW CONDITIONAL GO** for synthetic research, **NO-GO** for environmental/financial deployment.
