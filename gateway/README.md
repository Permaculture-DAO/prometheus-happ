# Prometheus Gateway — sensor → Holochain ingest (local-first)

Runs on the site gateway (e.g. the **Raspberry Pi 5** with Mosquitto). Subscribes to
MQTT sensor topics, builds a **provenance-anchored** MRV evidence for each reading,
and submits it to the **local** Holochain conductor (`create_evidence`). No public
write endpoint — sovereign / offline-first. This is the concrete "option A" ingest
path, matching the site plan (LoRaWAN → MQTT → Holochain).

## What it does (and does not)
- Records **evidence with provenance** (sensor+calibration hash, reading hash,
  timestamp). It does **not** create value, admissibility, or any right — the
  integrity zome rejects evidence missing provenance; the admissibility gate decides
  later (`assess_subject_admissibility`).
- **Test data is clearly labelled** (`TEST-` subject, `test:` id) — never mistaken
  for real evidence. The agent never fabricates real evidence.

## Topic convention
`prometheus/<subject_id>/<indicator>` → JSON
`{ sensor_id, value, unit, observed_at, calibration_hash?, confidence?, reviewer?, raw?, test? }`

## Run
```bash
npm install                 # mqtt + @holochain/client
npm run smoke               # pure pipeline test (no broker/conductor needed)
PROMETHEUS_DRY_RUN=1 npm start   # subscribe + build evidence, do NOT touch the conductor
npm start                   # full: needs a running conductor (hc-spin / hc sandbox)
```
Env: `MQTT_URL` (mqtt://127.0.0.1:1883), `MQTT_TOPIC` (prometheus/+/+),
`PROMETHEUS_SUBJECT_ID`, `PROMETHEUS_APP_ID`, `PROMETHEUS_ROLE` (hearth).

## Files
- `src/evidence.mjs` — pure provenance logic (dependency-free, unit-tested).
- `src/ingest.mjs` — MQTT → evidence → submit.
- `src/conductor.mjs` — `@holochain/client` boundary (lazy-loaded; dry-run skips it).
- `scripts/smoke.mjs` — pure-pipeline test.

## Status / next
The pure pipeline + dry-run are verified by `npm run smoke`. The live conductor
submit (`@holochain/client`) is verified against a running conductor (hc-spin /
sandbox) — the next integration step, when a conductor is up at the site. Reviewer
attests sensor calibration per batch before evidence clears Gate T.0.
