# Real-Data Evidence Integrity Gate

Status: BLOCKING assurance specification
Date: 2026-09-27

`PROMETHEUS_ALLOW_REAL` MUST remain `0` until this gate is implemented and verified end to end.

## Security and evidence boundary

The real-data gate must exist at the durable Holochain persistence/capability boundary. MQTT/gateway checks are defense in depth, not the authoritative admission control. A caller must not be able to bypass real-data authorization by invoking a zome or persistence function directly.

## Required controls

### 1. Persistence-bound real-data authorization

Every durable write classified as real data must fail closed unless the active approved real-data authorization state permits it. Synthetic/mock/test data must remain distinguishable from real observations.

### 2. Approved calibration registry

A submitted calibration hash is insufficient by itself. The admitted observation must bind `sensor_id` to the approved, frozen calibration artifact/hash valid for that sensor and collection interval. Unknown, expired, superseded or mismatched calibration must fail closed.

### 3. Authentication bound to routing identity

Message authentication must cover the complete security-relevant envelope, including at minimum subject identity, indicator/measurement routing, sensor identity, timestamp/sequence and payload digest. Authentication of payload bytes alone must not permit replay/rerouting under a different subject or indicator.

### 4. Independent reviewer attestation

Sensor/operator payloads must not self-declare admissibility by supplying reviewer text or reviewer identity. Reviewer attestation must be a separate authenticated workflow linked to immutable evidence identity, reviewer identity, decision, timestamp and lineage.

### 5. Deduplication identity

Gateway deduplication must include the full observation identity, including `subject_id` where subject changes meaning. Observations for distinct subjects must not collapse into one event.

### 6. Durable idempotency preserves sensor identity

Persistence idempotency must not collapse observations from distinct sensors merely because other dimensions match. Idempotency keys must preserve the scientifically meaningful identity dimensions and support at-least-once delivery safely.

## Verification matrix

Before enabling real collection, tests must demonstrate:

- direct zome/persistence bypass attempt -> rejected;
- unregistered calibration hash -> rejected;
- calibration valid for another sensor -> rejected;
- authenticated payload rerouted to another subject/indicator -> rejected;
- sensor-supplied reviewer/admissibility assertion -> no admissibility effect;
- same payload for two subjects -> two distinct observations;
- same measurement from two sensors -> two distinct durable observations;
- exact retry of the same immutable observation -> idempotent result;
- synthetic/mock event cannot be promoted silently to real;
- all rejection paths are observable/auditable without leaking secrets.

## Claim boundary

Passing this gate establishes engineering controls for real-data ingestion. It does not establish field validity, scientific admissibility, ecological outcome, certification, token entitlement, market admission or financial value.
