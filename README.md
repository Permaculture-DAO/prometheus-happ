# Prometheus hApp (Rust / Holochain)

> **Canonical repository of the Prometheus pilot hApp under the `Permaculture-DAO` organization.**  
> Founder and original authorship remain associated with [`Uwohali`](https://github.com/Uwohali).

This repository contains the **Holochain-native backend implementation** of the Prometheus pilot hApp.

It serves as the technical core of the Prometheus pilot stack and is intended to preserve:

- pilot-first scope
- semantic fidelity to the canonical domain model
- validation discipline
- bridge execution safety
- auditability
- future extensibility without contaminating pilot correctness

---

## Repository role

This repository is the **implementation home** of the Prometheus pilot backend.

It is responsible for:

- Holochain DNA and zome implementation
- canonical entity handling
- validation callbacks
- bridge control behavior
- pilot deployment hardening

This repository is **not** the place where pilot scope is redefined.

---

## Canonical status

This repository is the canonical implementation repository for the Prometheus pilot hApp.

If ambiguity exists between implementation and surrounding documentation, the following repositories define the canonical implementation contract and behavioral expectations:

- `prometheus-pilot-handoff-pack`
- `prometheus-mock-backend`
- `prometheus-ops-docs`
- `prometheus-evaluation-stack`

---

## Scope

### Included
- Rust / Holochain backend implementation
- zome logic
- integrity and coordinator structures
- validation behavior
- pilot-oriented backend runtime logic
- release and provenance continuity

### Excluded
- frontend/UI
- token market layer
- full DAO governance implementation
- RAP/SPV/securitization expansion
- full future financial layer
- sensor hardware deployment
- broader platform redesign

---

## Canonical specification

This hApp implements the backend rules defined in the canonical Prometheus specification.

Whitepaper repository:  
[`hearth-prometheus-whitepaper`](https://github.com/Uwohali/hearth-prometheus-whitepaper)

Canonical whitepaper release:  
[`v1.0.2`](https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/tag/v1.0.2)

---

## Repository structure

Core implementation artifacts currently include:

- `zome_coordinator`
- `zome_integrity`
- `zomes/coordinator/hearth_zome`
- `Cargo.toml`
- `Cargo.lock`
- `CONTRIBUTING.md`
- integrity/provenance companion files

These reflect the current implementation history and should not be casually restructured without explicit architectural review.

---

## Build and test

### Prerequisites
- Rust toolchain
- Holochain development environment

### Build

    cargo build

### Test

    cargo test

### Notes

The implementation should be interpreted as part of a pilot-first stack.  
Behavioral expectations are also defined in the related specification and mock repositories listed below.

---

## Related repositories

### `prometheus-pilot-handoff-pack`
Canonical implementation contract for the Prometheus Rigenera pilot.

### `prometheus-mock-backend`
Executable behavioral specification for the Prometheus pilot backend.

### `prometheus-ops-docs`
Operational SOPs, runbooks, troubleshooting, and execution discipline.

### `prometheus-evaluation-stack`
Scoring, reporting, and GO / NO_GO / MANUAL_REVIEW decisioning.

---

## Contributing

We welcome contributions from people aligned with the project’s principles and implementation discipline.

Please read [`CONTRIBUTING.md`](./CONTRIBUTING.md) before contributing.  
It contains governance expectations, constitutional guidelines, and commit-signing instructions.

---

## Release discipline

This repository preserves implementation continuity, release history, and provenance.

Canonical releases and integrity-related references should be treated as part of the project’s formal technical record.

---

## Whitepaper alignment

Whitepaper Version: `v1.0.2`

This implementation should remain aligned with the canonical whitepaper version unless an explicit architectural update is approved and documented.

---

## Current status

This repository contains real implementation work and active structural substance.

Prometheus is being developed with a **pilot-first discipline**, which means:

- explicit scope before expansion
- behavior before abstraction
- validation before execution
- operational clarity before scale
- implementation fidelity before architecture drift

---

## Founder and context

Prometheus and the h•eart•h intelligence initiative were founded by [`Uwohali`](https://github.com/Uwohali).

Canonical project development lives under the `Permaculture-DAO` organization.
