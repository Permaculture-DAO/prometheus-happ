# Prometheus hApp (Rust / Holochain)

> **Canonical repository of the Prometheus pilot hApp under the `Permaculture-DAO` organization.**  

This repository contains the Holochain-native backend implementation of the Prometheus pilot hApp.

It serves as the technical core of the Prometheus pilot stack and is intended to preserve:

- pilot-first scope
- semantic fidelity to the canonical domain model
- validation discipline
- backend runtime correctness
- auditability
- future extensibility without contaminating pilot correctness

## Repository role

This repository is the implementation home of the Prometheus pilot backend.

It is responsible for:

- Holochain DNA and zome implementation
- canonical entity handling
- validation callbacks
- pilot backend runtime behavior
- pilot deployment hardening

This repository is not the place where pilot scope is redefined.

Application-facing bridge logic and frontend console behavior are maintained in dedicated companion repositories.

## Runtime topology

The current Prometheus pilot stack is intentionally separated across bounded repositories.

This repository remains the canonical Holochain-native backend implementation layer.

Companion runtime-facing repositories include:

- `prometheus-bridge` — application-facing HTTP/API bridge in front of the Holochain websocket path
- `prometheus-console` — frontend console and public-facing application UI for the pilot stack

This separation exists to preserve backend correctness, pilot scope clarity, and architectural auditability.

## Canonical status

This repository is the canonical implementation repository for the Prometheus pilot hApp.

If ambiguity exists between implementation and surrounding documentation, the following repositories define the canonical implementation contract and behavioral expectations:

- `prometheus-pilot-handoff-pack`
- `prometheus-mock-backend`
- `prometheus-ops-docs`
- `prometheus-evaluation-stack`

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

## Canonical specification

This hApp implements the backend rules defined in the canonical Prometheus specification.

Whitepaper repository: `hearth-prometheus-whitepaper`  
Canonical whitepaper release: `v1.0.2`

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

For application-facing integration, the pilot runtime is expected to be reached through the websocket/app-interface path used by the bridge layer.

CLI-oriented local zome-call workflows may still be useful for debugging, but they should not be treated as the canonical frontend integration path for the pilot console.

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

### `prometheus-bridge`

Application-facing HTTP/API bridge for the Prometheus pilot runtime.

### `prometheus-console`

Frontend console repository for the Prometheus pilot application UI.

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

This implementation should remain aligned with canonical whitepaper release `v1.0.2` unless an explicit architectural update is approved and documented.

---

## Current runtime integration status

The pilot backend has validated a live websocket/app-interface integration path for runtime-facing message flow.

This confirms the current separation of concerns across:

- `prometheus-happ` as canonical backend runtime layer
- `prometheus-bridge` as application-facing bridge layer
- `prometheus-console` as frontend interaction layer

This repository should therefore be interpreted as the backend core of a layered pilot stack, not as a monolithic application repository.

This runtime validation clarifies current implementation topology and does not expand bounded pilot scope.

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

## Canonical texts

The Prometheus ecosystem is governed by layered canonical documentation.

For a compressed constitutional overview of the ecosystem, see:

- `docs/references/PROMETHEUS_CANON_12_PAGES.md`

This text complements the canonical white paper and does not expand the bounded scope of the current pilot implementation.

---

## Canonical document authority

Repository-bounded implementation in `prometheus-happ` does not redefine Prometheus constitutional meaning.

Canonical release freezes of the Prometheus document system are maintained in:

- [`prometheus-canonicals`](https://github.com/Permaculture-DAO/prometheus-canonicals)

Current canonical freeze reference:

- `releases/2026-04-09/`

---

## Founder and context

Prometheus and the h•eart•h intelligence initiative were founded by [`Uwohali`](https://github.com/Uwohali).

Canonical project development lives under the `Permaculture-DAO` organization.
