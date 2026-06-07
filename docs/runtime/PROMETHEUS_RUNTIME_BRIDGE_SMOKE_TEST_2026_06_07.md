# Prometheus Runtime Bridge Smoke Test — 2026-06-07

## Scope

This document records the successful HTTP bridge smoke test for the Prometheus Benchmark Intelligence Layer running through a live Holochain sandbox conductor.

## Toolchain

- Holochain: 0.6.1
- hc CLI: 0.6.1
- lair-keystore: 0.6.3
- Node.js: v18.19.1

## Runtime

- hApp ID: `hearth_prometheus`
- Admin port: `14600`
- App websocket port: `14602`
- DNA hash: `uhC0kIuwnPJ1OZx6ICpBo_Qg2NrMknkLcsI-AiWANuPDgKtyMvqxf`
- Zome: `zome_coordinator`
- Function: `hello_benchmark_layer`
- Bridge port: `8787`

## Endpoints

- `GET /health`
- `GET /smoke-test`

## Verified result

The `/smoke-test` endpoint returned:

{
  "status": "ok",
  "layer": "prometheus-runtime-bridge",
  "mode": "zome_call_via_hc_cli_with_passphrase",
  "app_id": "hearth_prometheus",
  "holochain_admin_port": 14600,
  "holochain_app_port": 14602,
  "zome": "zome_coordinator",
  "function": "hello_benchmark_layer",
  "holochain_result": "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification",
  "expected_result": "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification"
}

## Boundary

This is a runtime smoke test, not a certification engine.

Prometheus currently issues evidence-based runtime evaluation records. Certification, accreditation, and institutional validation require an external authority, institutional partner, or formally governed certification body.

## Status

PASSED.
