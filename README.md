# Prometheus hApp — Genesis Runtime

**Prometheus v1.1 — Genesis Institutional Canonical Line**

This repository is the clean Holochain-native pilot runtime repository for the Prometheus v1.1 Genesis line.

The primary canonical source repository is:

**Permaculture-DAO/prometheus-canon**

This repository implements bounded pilot runtime behavior.

It does not define Prometheus constitutional meaning.

## Repository status

Current status:

**Genesis runtime staging — pre-final-freeze, pre-runtime-release**

This repository is not yet a final runtime release.

No `.dna`, `.happ`, `.webhapp`, `.sha256`, or `.asc` file should be treated as final unless generated after runtime QA, canonical boundary review, checksum regeneration, and authorized release signing.

## Runtime role

This repository is expected to contain the Holochain-native backend implementation layer for the Prometheus pilot stack, including:

- DNA;
- integrity zomes;
- coordinator zomes;
- validation-bound records;
- source-chain-based runtime behavior;
- bounded pilot backend logic;
- runtime integration notes.

## Canonical boundary

The White Paper and `prometheus-canon` define meaning.

This repository implements behavior.

Runtime code, zome functions, hApp manifests, tests, build artifacts, or generated outputs must not silently redefine:

- PRU logic;
- RAP inclusion;
- TRBK role;
- HoloFuel role;
- investor exposure;
- land rights;
- legal meaning;
- governance authority;
- ecological credit issuance;
- capital-facing claims.

## Legacy boundary

The previous pre-v1.1 runtime repository has been moved to:

**Permaculture-DAO/legacy-prometheus-happ-pre-v1.1**

Legacy code may inform this repository.

Legacy code does not govern this repository.

Any legacy module, zome, manifest, build configuration, or runtime behavior must be explicitly reviewed before reintroduction into this Genesis runtime line.

## Pilot-first discipline

This repository follows pilot-first discipline:

- explicit scope before expansion;
- behavior before abstraction;
- validation before execution;
- runtime clarity before scale;
- implementation fidelity before architecture drift;
- canonical boundary before runtime claim.

## Related repositories

Primary canonical source:

- `Permaculture-DAO/prometheus-canon`

Runtime-facing repositories:

- `Permaculture-DAO/prometheus-bridge`
- `Permaculture-DAO/prometheus-console`

Operational and evaluation repositories:

- `Permaculture-DAO/prometheus-ops-docs`
- `Permaculture-DAO/prometheus-evaluation-stack`
- `Permaculture-DAO/prometheus-pilot-handoff-pack`
- `Permaculture-DAO/prometheus-mock-backend`

## Operational rule

Runtime behavior lives here.

Canonical meaning lives in `prometheus-canon`.

Bridge/API behavior lives in `prometheus-bridge`.

Frontend behavior lives in `prometheus-console`.

Legacy runtime history remains outside the active path unless formally reintroduced.
