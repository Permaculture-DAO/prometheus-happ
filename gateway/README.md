# Prometheus Gateway — sensor → Holochain ingest (local-first)

Runs on the site gateway (e.g. the **Raspberry Pi 5** with Mosquitto). Subscribes to
MQTT sensor topics, builds **provenance-anchored** MRV evidence for each reading, and
submits it to the **local** Holochain conductor (`create_evidence`). No public write
endpoint — sovereign / offline-first. This is the concrete ingest path matching the site
plan (LoRaWAN → MQTT → Holochain).

## What it does (and does not)
- Records **evidence with provenance** (sensor + calibration/method hash, reading hash,
  timestamp). It does **not** create value, admissibility, certification or any right.
- **TEST data is clearly labelled** (`TEST-` subject, `test:` id) and cannot be mistaken
  for real evidence. The gateway never fabricates real evidence.
- The first permitted real evidence is the frozen-protocol **baseline**. Collection is
  separate from reviewer attestation and Gate T.0 admissibility.

## Hardening (fail-closed)
- **Validation:** rejects unsupported indicators, non-integer/out-of-range timestamps,
  non-finite confidence, malformed ids/topics, non-JSON payloads and invalid ranges.
- **Synthetic-until-authorized guard:** unless `PROMETHEUS_ALLOW_REAL=1`, every reading is
  forced to TEST.
- **Real subject allowlist:** real mode requires `PROMETHEUS_ALLOWED_SUBJECTS`; a real
  reading for any other subject is quarantined.
- **Real method provenance:** non-TEST sensor evidence requires `calibration_hash` as a
  SHA-256 hex digest. The TEST-only sensor-id fallback is never used for real evidence.
- **Idempotency:** duplicate evidence ids are deduplicated, not re-submitted.
- **Quarantine:** malformed messages are counted + logged (code only), never crash the
  loop. **Bounded retry** on transient submit errors; **fail-closed** if the app role/cell
  is absent. Logs status only — **never raw payloads or credentials**.

## Topic convention
`prometheus/<subject_id>/<indicator>` → JSON
`{ sensor_id, value, unit, observed_at, calibration_hash?, confidence?, reviewer?, raw?, test?, signature? }`

## Run
```bash
npm ci
npm run smoke
PROMETHEUS_DRY_RUN=1 npm start   # subscribe + build evidence, do NOT touch the conductor
npm start                         # full: needs a running conductor
```

Important env: `MQTT_URL`, `MQTT_TOPIC`, `MQTT_USERNAME`/`MQTT_PASSWORD`,
`PROMETHEUS_SUBJECT_ID`, `PROMETHEUS_ALLOWED_SUBJECTS`, `PROMETHEUS_APP_ID`,
`PROMETHEUS_ROLE`, `PROMETHEUS_ALLOW_REAL`, `PROMETHEUS_INDICATORS`,
`PROMETHEUS_REQUIRE_SIGNATURE`, `PROMETHEUS_HMAC_KEY`, `PROMETHEUS_MAX_RETRIES`.

## Real-data collection gate
Keep `PROMETHEUS_ALLOW_REAL=0` through engineering and physical TEST commissioning. Set it
to `1` only after the independent reviewer is appointed, OHE/comparator/dates are fixed,
the pre-registration is SHA256-recorded + GPG-signed, the actual sensor path has passed
TEST-only commissioning, and `PROMETHEUS_ALLOWED_SUBJECTS` exactly matches the frozen field
design. The baseline is then the first real evidence acquired. Reviewer batch attestation
is required before that evidence may clear Gate T.0.

## Authenticity
Field MQTT uses authenticated credentials. Optional HMAC payload verification is supported
with `PROMETHEUS_REQUIRE_SIGNATURE=1` and `PROMETHEUS_HMAC_KEY`; when enabled, unsigned or
invalid payloads are quarantined. Secrets remain outside Git.

## Files
- `src/evidence.mjs` — pure provenance + validation logic.
- `src/ingest.mjs` — MQTT → evidence → submit, including real-mode subject policy.
- `src/conductor.mjs` — `@holochain/client` boundary.
- `scripts/smoke.mjs` — pure-pipeline hardening tests.

## Status / next
The pipeline and dry-run are verified by `npm run smoke`; live conductor integration has
separate runtime proof. The field pilot remains PRE-PILOT until the real-world freeze
conditions are satisfied and a real baseline is collected. Runtime readiness is not field
validation. Independent reviewer attestation governs evidence batches before Gate T.0.
