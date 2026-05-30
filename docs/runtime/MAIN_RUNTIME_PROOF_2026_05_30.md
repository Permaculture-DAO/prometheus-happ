# Main Runtime Proof — 2026-05-30

## Scope

This document records the successful local Holochain runtime activation of the Prometheus Benchmark Intelligence Layer.

## Toolchain

- Holochain: 0.6.1
- hc CLI: 0.6.1
- lair-keystore: 0.6.3

## Runtime Ports

- Admin websocket port: 14600
- App websocket port: 14602

## Application

- hApp ID: hearth_prometheus
- Role: hearth
- Zome: zome_coordinator
- Function: hello_benchmark_layer
- Payload: null

## Verified Cell

- DNA hash: uhC0kIuwnPJ1OZx6ICpBo_Qg2NrMknkLcsI-AiWANuPDgKtyMvqxf

## Verified Runtime Result

    "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification"

## Boundary

This runtime establishes the Holochain-native benchmark foundation.

It does not yet implement a full scoring engine, certification authority, accreditation layer, or automated evaluation pipeline.

Prometheus issues evidence-based evaluation records.

Certification and accreditation require a recognized external authority, institutional partner, or formally governed certification body.

## Runtime Verification Commands

    hc sandbox call --running 14600 list-apps
    hc sandbox call --running 14600 list-cells
    hc sandbox call --running 14600 add-app-ws 14602
    hc sandbox zome-call-auth --running 14600 hearth_prometheus
    hc sandbox zome-call --running 14600 hearth_prometheus uhC0kIuwnPJ1OZx6ICpBo_Qg2NrMknkLcsI-AiWANuPDgKtyMvqxf zome_coordinator hello_benchmark_layer null

## Status

PASSED.
