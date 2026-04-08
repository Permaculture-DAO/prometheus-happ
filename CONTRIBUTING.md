# CONTRIBUTING.md — Technical Contribution Guide for `Prometheus-happ`

Welcome, builder.

This repository contains the **Holochain-native backend implementation** of the Prometheus pilot hApp under the `Permaculture-DAO` organization.

This file explains **how to contribute safely and coherently** to this repository.

It is not a replacement for the White Paper or the broader Prometheus architecture materials.  
It is the contribution guide for this specific codebase.

---

## 1. Purpose of this repository

`Prometheus-happ` is the technical implementation repository for the **Prometheus pilot backend**.

Its focus is:

- Rust / Holochain backend implementation
- zome and integrity logic
- validation behavior
- pilot-first backend runtime structure
- implementation continuity and release provenance

This repository is **not** the place to redesign the full Prometheus ecosystem.

---

## 2. Read this before contributing

Before opening a pull request, read these materials in this order:

### Core repository documentation
1. `README.md`
2. this `CONTRIBUTING.md`

### Canonical surrounding repositories
3. `prometheus-pilot-handoff-pack`
4. `prometheus-mock-backend`
5. `prometheus-ops-docs`
6. `prometheus-evaluation-stack`

### Canonical specification
7. Whitepaper repository:  
   [`hearth-prometheus-whitepaper`](https://github.com/Uwohali/hearth-prometheus-whitepaper)

8. Canonical whitepaper release:  
   [`v1.0.2`](https://github.com/Uwohali/hearth-prometheus-whitepaper/releases/tag/v1.0.2)

If you are unclear on architecture, validation semantics, or pilot scope, stop and review those sources first.

---

## 3. Contribution philosophy

Prometheus is being developed with a **pilot-first discipline**.

This means contributions must favor:

- semantic fidelity over reinterpretation
- explicit validation over hidden assumptions
- reviewability over unnecessary abstraction
- pilot correctness over future-scope expansion
- architectural discipline over improvisation

A contribution is valuable only if it strengthens the pilot without destabilizing the canonical structure.

---

## 4. Scope for this repository

### In scope
Contributions are welcome for:

- Rust implementation quality
- Holochain zome logic
- integrity/coordinator behavior
- validation callbacks
- code clarity
- tests
- documentation directly relevant to this repository
- build/test reproducibility
- release hygiene

### Out of scope
Do **not** use this repository to introduce or silently redesign:

- frontend/UI layers
- mobile/Volla stack
- full DAO/governance expansion
- off-chain AGI platform redesign
- token market system redesign
- full financial architecture
- unrelated manifesto/political insertions
- architecture changes that belong in the handoff pack or whitepaper

If a change affects canonical scope or semantics, it must be proposed explicitly before implementation.

---

## 5. Constitutional constraints

Every contribution must respect the Prometheus constitutional frame.

At minimum, contributors must preserve:

- **Agent sovereignty**
- **Non-commodification of life**
- **Planetary boundary respect**
- **Pilot-first discipline**
- **Validation before execution**
- **Responsibility and maturity in implementation**

If your code weakens these principles, it should not be merged.

---

## 6. Verification of canonical documents

Canonical documents should be treated as integrity-sensitive sources.

If your contribution depends on whitepaper-level interpretation, you are expected to verify the source materials using the project’s cryptographic integrity workflow where relevant.

At minimum, check:

- repository source
- release/tag alignment
- file integrity when using signed artifacts

This is not bureaucracy.  
It is part of the project’s source-of-truth discipline.

---

## 7. Coding expectations

### General expectations
Contributions should be:

- small enough to review
- explicit in intent
- documented when behavior changes
- aligned with repository scope
- traceable to a technical reason

### Avoid
Do not submit contributions that are:

- vague
- architecture-expanding without approval
- semantically ambiguous
- stylistic churn without functional value
- bulk rewrites without justification

---

## 8. Validation expectations

If you modify behavior, validation, or semantics, you must explain:

- what changed
- why it changed
- what canonical rule or repository it aligns with
- how it was tested

Special attention is required for changes affecting:

- entry validation
- agent permissions
- covenant enforcement
- event/resource semantics
- bridge or execution safety
- integrity-related flows

---

## 9. Build and test expectations

Before submitting a PR, ensure that the code can still be built and tested cleanly.

Minimum expectations:

### Build

    cargo build

### Test

    cargo test

If your change affects validation logic, include or update tests accordingly.

---

## 10. Pull request rules

Every pull request should be:

- scoped
- clearly described
- technically justified
- consistent with pilot-first architecture

### PRs should include
- a short summary
- the reason for the change
- affected components
- any architectural implications
- testing notes

### PRs should not include
- silent scope expansion
- undocumented semantic changes
- large mixed-purpose edits
- unrelated documentation ideology dumps

---

## 11. Required PR checklist

Before opening a PR, confirm all of the following:

- [ ] I read `README.md`
- [ ] I read `CONTRIBUTING.md`
- [ ] My change is in scope for `Prometheus-happ`
- [ ] I did not silently redefine canonical project semantics
- [ ] I did not expand the pilot into future layers
- [ ] I explained the technical reason for the change
- [ ] `cargo build` passes
- [ ] `cargo test` passes or I explained why not
- [ ] I added or updated tests where behavior changed
- [ ] I documented any validation-sensitive change
- [ ] My contribution improves clarity, correctness, or implementation quality

---

## 12. When to open an issue first

Open an issue before coding if your change would:

- affect canonical architecture
- change domain semantics
- alter validation rules
- broaden scope
- add new subsystem boundaries
- require coordination across repositories

If you are unsure whether the change is local or architectural, assume it is architectural and ask first.

---

## 13. Related repositories

This repository does not stand alone.

### `prometheus-pilot-handoff-pack`
Canonical implementation contract for the pilot.

### `prometheus-mock-backend`
Executable behavioral specification.

### `prometheus-ops-docs`
Operational discipline and runbooks.

### `prometheus-evaluation-stack`
Scoring, reporting, and GO / NO_GO logic.

If your contribution would be better placed in one of those repositories, do not force it into `Prometheus-happ`.

---

## 14. Commit hygiene

Use commit messages that are:

- specific
- readable
- technically meaningful

Examples:

- `Fix README markdown formatting and align repository documentation`
- `Add validation guard for covenant-dependent entry creation`
- `Refactor zome module imports for clarity`

Avoid messages like:
- `update`
- `misc fixes`
- `stuff`
- `final`

---

## 15. Maintainer review criteria

Maintainers should review contributions against these questions:

1. Is the change in scope?
2. Is the technical intent clear?
3. Does it preserve pilot-first discipline?
4. Does it preserve canonical semantics?
5. Is it testable and reviewable?
6. Does it reduce or increase ambiguity?
7. Is it the right change for this repository?

If the answer to several of these is “no”, the PR should not be merged.

---

## 16. Founder and canonical context

Prometheus and the h•eart•h intelligence initiative were founded by [`Uwohali`](https://github.com/Uwohali).

Canonical project development lives under the `Permaculture-DAO` organization.
