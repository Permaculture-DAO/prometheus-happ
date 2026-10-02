# Real-Data Control Plane v1

Status: **DRAFT — engineering control plane only**
Base: PR #32 (`assurance/persistence-real-data-gate-v1`)
REAL collection/persistence: **NO-GO**

## Purpose

This layer introduces persistent control-plane types without opening the real-data write path.

It adds:

- a DNA-bound `real_data_enabled` switch;
- an optional DNA-bound real-data authority agent;
- a DNA-bound list of approved reviewer agents;
- authority-authored real-data authorization entries;
- authority-authored calibration approval/revocation entries;
- reviewer-authored independent review attestations linked to immutable evidence actions;
- a read-only gate-status extern.

## Fail-closed defaults

The DNA manifest ships with:

- `real_data_enabled: false`;
- `real_data_authority: null`;
- `approved_reviewers: []`.

Therefore control-plane writes are rejected by integrity validation unless a new DNA is explicitly provisioned with an authority. Existing PR #32 behaviour remains unchanged: REAL evidence itself is still rejected at the integrity and coordinator persistence boundary.

## Integrity rules

Authorization and calibration entries must be authored by the agent configured in DNA properties. A payload cannot self-declare its authority.

Review attestations must:

- be authored by the same agent named as reviewer;
- use an agent present in `approved_reviewers`;
- target a valid MRV evidence action;
- be authored by an agent different from the evidence author;
- carry an explicit ACCEPT/REJECT decision;
- carry a 64-hex lineage hash.

These checks live in the integrity zome, not only in a replaceable gateway.

## Verification completed

- `cargo test -p zome_integrity --locked`: **31/31 PASS**;
- `cargo build --locked --target wasm32-unknown-unknown --workspace`: **PASS**.

The first compile exposed a review-target decode error-conversion bug; it was corrected before this record.

## What this does not yet prove

This PR does **not**:

- enable REAL evidence persistence;
- prove a production capability-grant deployment;
- configure a real authority key;
- configure real reviewer keys;
- prove calibration supersession/revocation semantics end to end;
- migrate the live DNA;
- establish scientific admissibility, field validity, certification, legal admission, token rights or value.

## Next acceptance gate

Before REAL can be considered for activation:

1. provision a review-only test DNA with explicit authority/reviewer keys;
2. add conductor tests showing unauthorized control entries are rejected;
3. add conductor tests for independent reviewer enforcement;
4. define and test calibration supersession/revocation semantics;
5. bind a REAL observation to an active authorization and calibration state;
6. run the full #30 verification matrix;
7. produce a new DNA hash and migration/release record;
8. obtain steward approval before any live migration.

Until then `PROMETHEUS_ALLOW_REAL=0` and the DNA-level REAL gate remains closed.
