# Release Notes — `v1.1.3-runtime-proof` (CANDIDATE)

A **runtime** increment over `v1.1.2-genesis`. The governing prose canon (white paper)
is **unchanged**; this release advances only the Holochain runtime + the sensor gateway,
and adds the bounded synthetic vertical proof. No new external-facing claim of validation,
value, or rights — admissibility remains **binary**, value stays suspended at zero until
Gate T.0 on real gated evidence.

## What's new since v1.1.2-genesis
- **Evidence spine, live-proven.** Conductor-side core verified LIVE and machine-checked:
  `create_ohe` → provenance-anchored `create_evidence` → `assess_subject_admissibility`
  (binary, no value) → integrity `validate` rejecting invalid entries.
- **Sensor → Holochain ingest gateway** (`gateway/`): MQTT/Mosquitto → `create_evidence`
  on the local conductor (local-first, sovereign, no public write endpoint).
- **Gateway hardening:** fail-closed input validation; synthetic-until-authorized guard
  (`PROMETHEUS_ALLOW_REAL`); **idempotent** create (`create_evidence_idempotent`);
  quarantine + bounded retry; **HMAC** reading signatures (`signReading`/verify);
  structured events; qos:1; broker reconnect handling; safe logging.
- **Reproducibility:** exact-pinned gateway deps + committed lockfile + Node ≥20.
- **Operator console UI** (`ui/`): green-on-black terminal; LIVE (launcher-signed) and
  SIMULATION modes; `@holochain/client` **vendored locally** (no CDN/CSP fragility).
- **WP4 B0–B8** bounded synthetic vertical proof (MQTT→gateway→Holochain, idempotency,
  restart, second-agent matching record) — recorded with toolchain + artefact hashes.

## Scope / boundary
See `CLAIM_BOUNDARY.md`. Synthetic TEST data only; no empirical/site validation; not an
offer; PRU is not a right, RAP is not a security. Prose canon remains v1.1.2-genesis.

## Verify
Reproduce on the pinned toolchain from commit `4679409`; confirm the artefact SHA256 in
`RELEASE_MANIFEST.md`; run `gateway` unit tests (`npm ci && npm run smoke`) and the WP4
harness (`gateway/scripts/run_wp4.sh`). Signatures per `SIGNING_INSTRUCTIONS.md`.
