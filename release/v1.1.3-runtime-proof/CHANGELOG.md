# Changelog — v1.1.3-runtime-proof (CANDIDATE)

Runtime-only increment over v1.1.2-genesis (prose canon unchanged).

- Evidence spine proven LIVE (OHE → evidence → binary admissibility → validate rejects invalid).
- Sensor→Holochain ingest gateway (MQTT/Mosquitto → create_evidence, local-first).
- Gateway hardening: validation, synthetic-until-authorized guard, idempotent create,
  quarantine, bounded retry, HMAC reading signatures, qos:1, reconnect, safe logging.
- Reproducibility: exact-pinned deps + committed lockfile + Node ≥20.
- Operator console UI (terminal; LIVE + SIMULATION; @holochain/client vendored, no CDN).
- WP4 B0–B8 bounded synthetic vertical proof recorded (toolchain + artefact hashes).

Source: prometheus-happ @ 4679409. Artefacts + hashes: RELEASE_MANIFEST.md.
