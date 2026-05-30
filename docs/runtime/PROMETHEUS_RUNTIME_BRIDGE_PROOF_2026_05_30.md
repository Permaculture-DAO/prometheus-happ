# Prometheus Runtime Bridge Proof — 2026-05-30

## Scope

This document records the first successful HTTP bridge health endpoint running above the local Holochain runtime for the Prometheus Benchmark Intelligence Layer.

## Runtime Stack

- Holochain conductor: live
- Holochain admin websocket: 14600
- Holochain app websocket: 14700
- hApp ID: hearth_prometheus
- Role: hearth
- Zome: zome_coordinator
- Smoke function: hello_benchmark_layer
- Bridge HTTP port: 8787
- Bridge endpoint: /health

## Verified Holochain Runtime Result

The underlying Holochain runtime smoke test returned:

    "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification"

## Verified HTTP Bridge Health Result

The HTTP bridge returned:

    {
      "status": "ok",
      "layer": "prometheus-runtime-bridge",
      "mode": "health_only_admin_api",
      "app_id": "hearth_prometheus",
      "holochain_admin_port": 14600,
      "holochain_app_port": 14700,
      "app_installed": true,
      "dna_present": true,
      "app_websocket_present": true,
      "zome": "zome_coordinator",
      "smoke_function": "hello_benchmark_layer",
      "verified_runtime_result": "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification"
    }

## Boundary

This bridge validates the live Holochain conductor, installed hApp, cell DNA and app websocket through the Holochain admin API.

This is a health-only bridge. Direct zome-call integration through an application websocket remains a next-step bridge iteration.

Prometheus issues evidence-based evaluation records.

Certification and accreditation require a recognized external authority, institutional partner, or formally governed certification body.

## Status

PASSED.
