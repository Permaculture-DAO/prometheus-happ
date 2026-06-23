# Claim Boundary — `v1.1.3-runtime-proof`

The class of claim is bounded by the lowest unresolved gate. This release concerns
**runtime behaviour with synthetic data only** — never empirical pilot validation.

## May be claimed
> On the pinned toolchain, the runtime reproducibly ingests labelled **synthetic TEST**
> evidence (MQTT→gateway→conductor), persists it with provenance, survives gateway and
> conductor restart, deduplicates idempotently, and computes **binary** admissibility
> (false when any configured gate is missing; true only when all pass). Integrity
> validation rejects invalid entries. Verified by WP4 B0–B8.

## MUST NOT be claimed
- Any empirical/agronomic result, site validation, or "validated at our site".
- Any PRU value, RAP inclusion, certification, security, or financial admissibility.
- "Production-ready", "certified", "guaranteed", "risk-free", "investable".
- Any use of real or personally-identifying data (synthetic only).
- Any change to the **prose canon** — it remains `v1.1.2-genesis`.

## Separation (human-authorised, not granted by this package)
GPG signing, tagging, GitHub release, steward ratification, production deployment, and
use of real field evidence are separate human actions. This candidate authorises none.
