# Prometheus Local Runtime Runbook — 2026-06-07

## Purpose

This runbook describes the local runtime procedure for the Prometheus hApp, Holochain sandbox conductor, runtime bridge, and smoke-test endpoint.

It is intended for controlled local development and runtime verification.

## Runtime stack

- Holochain: 0.6.1
- hc CLI: 0.6.1
- lair-keystore: 0.6.3
- Node.js: v18.19.1

## Ports

- Holochain admin websocket: 14600
- Holochain app websocket: 14602
- Prometheus runtime bridge: 8787

## App identifiers

- hApp ID: hearth_prometheus
- Zome: zome_coordinator
- Smoke-test function: hello_benchmark_layer
- Expected result: Prometheus Benchmark Intelligence Layer online: evaluation_not_certification

## 1. Build and pack artifacts

From the repository root:

    cargo check
    cargo build --target wasm32-unknown-unknown

    mkdir -p dnas/hearth/wasm

    cp target/wasm32-unknown-unknown/debug/zome_integrity.wasm dnas/hearth/wasm/zome_integrity.wasm
    cp target/wasm32-unknown-unknown/debug/zome_coordinator.wasm dnas/hearth/wasm/zome_coordinator.wasm

    rm -f dnas/hearth/hearth.dna hearth_prometheus.happ hearth_prometheus_web.webhapp ui.zip

    (cd ui && zip -r ../ui.zip .)

    hc dna pack dnas/hearth
    hc app pack .
    hc web-app pack .

## 2. Create a fresh sandbox

    rm -rf .runtime/sandbox-main-runtime
    mkdir -p .runtime/sandbox-main-runtime

    cp hearth_prometheus.happ .runtime/sandbox-main-runtime/

    cd .runtime/sandbox-main-runtime

    hc sandbox generate --app-id hearth_prometheus hearth_prometheus.happ

## 3. Start the Holochain conductor

In terminal 1:

    cd "/mnt/d/hearth intelligence – Prometheus Sovereign Syntropy/documenti/White Paper/GENESIS_REINSTALL_V1_1/repos/prometheus-happ/.runtime/sandbox-main-runtime"

    hc sandbox -f 14600 run

Keep this terminal open.

## 4. Attach app websocket and authorize zome calls

In terminal 2:

    cd "/mnt/d/hearth intelligence – Prometheus Sovereign Syntropy/documenti/White Paper/GENESIS_REINSTALL_V1_1/repos/prometheus-happ/.runtime/sandbox-main-runtime"

    hc sandbox call --running 14600 list-apps
    hc sandbox call --running 14600 list-cells
    hc sandbox call --running 14600 list-app-ws

If the app websocket is missing:

    hc sandbox call --running 14600 add-app-ws 14602
    hc sandbox call --running 14600 list-app-ws

Authorize zome calls:

    hc sandbox zome-call-auth --running 14600 hearth_prometheus

## 5. Direct Holochain smoke test

Use the DNA hash returned by list-cells:

    hc sandbox zome-call --running 14600 hearth_prometheus <DNA_HASH> zome_coordinator hello_benchmark_layer null

Expected result:

    Prometheus Benchmark Intelligence Layer online: evaluation_not_certification

## 6. Start the runtime bridge

In terminal 3, from the repository root:

    cd "/mnt/d/hearth intelligence – Prometheus Sovereign Syntropy/documenti/White Paper/GENESIS_REINSTALL_V1_1/repos/prometheus-happ"

    unset PROMETHEUS_HC_PASSPHRASE

    read -s -p "Enter Holochain zome-call passphrase: " PROMETHEUS_HC_PASSPHRASE
    echo

    export PROMETHEUS_HC_PASSPHRASE
    export HOLOCHAIN_ADMIN_PORT=14600
    export HOLOCHAIN_APP_PORT=14602
    export PROMETHEUS_BRIDGE_PORT=8787

    node bridge/server.mjs

Keep this terminal open.

## 7. HTTP smoke test

In terminal 4:

    curl -s http://127.0.0.1:8787/health | python3 -m json.tool

    curl -s http://127.0.0.1:8787/smoke-test | python3 -m json.tool

Expected smoke-test result:

    status: ok
    holochain_result: Prometheus Benchmark Intelligence Layer online: evaluation_not_certification
    expected_result: Prometheus Benchmark Intelligence Layer online: evaluation_not_certification

## 8. Shutdown

Stop the bridge:

    Ctrl+C

Stop the conductor:

    Ctrl+C

Optional cleanup:

    pkill -f "node bridge/server.mjs" || true
    pkill -f "holochain" || true
    pkill -f "lair-keystore" || true
    pkill -f "hc sandbox" || true

## Boundary

This local runtime verifies Holochain connectivity, hApp installation, app websocket availability, zome-call authorization, bridge availability, and smoke-test execution.

It is not a certification engine, accreditation layer, or automated benchmark scoring system.

## Status

ACTIVE LOCAL RUNTIME RUNBOOK.
