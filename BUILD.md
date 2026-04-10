# Prometheus hApp Build and Runtime Guide

## Purpose

This document defines the canonical build flow, artifact flow, and supported runtime profiles for the Prometheus Holochain application.

The signed artifacts are runtime-agnostic.
They may be executed through:

1. direct `hc sandbox` commands
2. persistent `tmux`-managed CLI runtime
3. Holochain Launcher / Moss GUI runtime

The artifact set remains the same across all execution modes.

---

## Repository scope

Primary workspace:

- `~/prometheus-happ`

Core files:

- `Cargo.toml`
- `dnas/hearth/dna.yaml`
- `happ.yaml`
- `web-happ.yaml`

Build outputs:

- `workdir/dna/hearth.dna`
- `workdir/app/hearth_prometheus.happ`

Deployment mirror on D drive:

- `/mnt/d/hearth intelligence – Prometheus Sovereign Syntropy/documenti/Holochain`

---

## Build prerequisites

Required tools:

- Rust toolchain
- `wasm32-unknown-unknown` target
- `hc` CLI
- `lair-keystore`
- `tmux` (optional but recommended for persistent CLI runtime)

Example checks:

```bash
command -v cargo
command -v hc
command -v lair-keystore
rustup target list --installed | grep wasm32-unknown-unknown
