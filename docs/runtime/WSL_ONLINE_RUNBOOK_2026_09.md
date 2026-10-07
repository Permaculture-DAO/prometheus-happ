# Prometheus WSL Online Runtime Runbook — 2026-09

## Scope

This runbook prepares the Prometheus Genesis runtime for controlled operation on Linux under WSL. It is an engineering/deployment document only.

The runtime topology is:

`public browser -> local ingress -> prometheus-bridge -> loopback runtime adapter -> Holochain conductor`

The Holochain admin websocket is never a public endpoint.

## Repository layout

The runner expects sibling repositories under one workspace directory:

- `prometheus-happ`
- `prometheus-bridge`
- `prometheus-console`

Override paths with `PROMETHEUS_WORKSPACE_ROOT`, `PROMETHEUS_BRIDGE_ROOT`, or `PROMETHEUS_CONSOLE_ROOT` when needed.

## Pinned Genesis runtime line

The automated runner currently requires:

- Holochain `0.6.1`
- `hc` `0.6.1`
- the existing `hearth_prometheus` hApp line

The runner fails closed on a different Holochain/hc version. Version migration is a separate engineering change because runtime/tooling upgrades must not silently alter the signed Genesis line.

## Ports

All Holochain-sensitive interfaces remain loopback-only:

- `14600` — Holochain admin websocket; never expose publicly
- `14602` — Holochain app websocket; never expose directly from this topology
- `8788` — local Holochain runtime adapter; loopback only
- `8787` — read-only Prometheus public bridge; loopback behind ingress
- `8080` — local web ingress for the public console and `/api/*`

Only the ingress target is intended to be mapped by an external HTTPS tunnel/reverse proxy.

## Secret handling

Set `PROMETHEUS_HC_PASSPHRASE` in the WSL process environment or enter it interactively when the runner starts. Do not commit it, place it in the repository, print it to logs, or expose it to the browser.

## Start

From `prometheus-happ`:

```bash
chmod +x scripts/runtime/wsl_online_runner.sh scripts/runtime/stop_wsl_online_runner.sh
./scripts/runtime/wsl_online_runner.sh
```

Set `PROMETHEUS_REBUILD=1` only when an intentional rebuild is required.

The runner:

1. verifies the WSL/Linux toolchain and pinned Holochain line;
2. builds the hApp only when the bundle is missing or rebuild is explicitly requested;
3. creates a persistent sandbox only when one does not already exist;
4. starts the conductor on loopback;
5. ensures the app websocket exists;
6. starts the loopback runtime adapter;
7. starts the separate read-only public bridge;
8. starts the Caddy ingress serving `prometheus-console` and proxying `/api/*`;
9. verifies the end-to-end `/api/runtime` route.

Successful completion ends with `RUNNER READY`.

## Stop

```bash
./scripts/runtime/stop_wsl_online_runner.sh
```

The stop script terminates only PIDs recorded by this runner.

## Public domain handoff

After the runner is healthy, the external HTTPS ingress/tunnel may map `app.heart-intelligence.earth` to `127.0.0.1:8080` from the same WSL environment.

Do not map ports `14600`, `14602`, or `8788` to the public network. Holochain documents the application interface as local to the participant runtime; the public Prometheus page therefore consumes a sanitized read-only bridge rather than exposing the conductor directly.

## Runtime status boundary

`/api/runtime` reports technical connectivity only: hApp presence, expected DNA presence, and app websocket availability. It does not establish field validation, scientific validation, certification, pilot completion, ecological performance, legal admission, or institutional acceptance.
