# Genesis Benchmark Runtime Proof — 2026-05-30

## Scope

This document records the first successful Holochain runtime smoke test for the Prometheus Benchmark Intelligence Layer.

## Toolchain

- Holochain: 0.6.1
- hc CLI: 0.6.1
- lair-keystore: 0.6.3

## Application

- hApp ID: hearth_prometheus
- Role: hearth
- Zome: zome_coordinator
- Function: hello_benchmark_layer
- Payload: null

## Runtime result

The zome call returned:

`"Prometheus Benchmark Intelligence Layer online: evaluation_not_certification"`

## Boundary

This proof confirms runtime execution of a Benchmark Intelligence smoke-test function.

It does not yet represent a full benchmark engine, scoring protocol, certification system, accreditation layer, or automated evaluation pipeline.

Prometheus issues evidence-based evaluation records.

Certification and accreditation require a recognized external authority, institutional partner, or formally governed certification body.

## Technical notes

The runtime required:

- Holochain / hc alignment to 0.6.1;
- HDK crate alignment to 0.6.1;
- WASM build configuration for getrandom on wasm32-unknown-unknown;
- fresh DNA / hApp repackaging;
- fresh sandbox generation;
- zome-call authorization via capability grant.

## Verified runtime path

- Sandbox: sandbox-benchmark-061-fresh
- Admin port: 14600
- App interface port: 14601
- hApp ID: hearth_prometheus
- DNA hash: uhC0kIuwnPJ1OZx6ICpBo_Qg2NrMknkLcsI-AiWANuPDgKtyMvqxf
- Zome: zome_coordinator
- Function: hello_benchmark_layer
- Payload: null

## Status

PASSED.
