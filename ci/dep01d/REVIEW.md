# DEP-01D — Independent runner gate for deployed WASM (review-only)

Status: **DRAFT / NOT MERGED / NOT A SIGNED RELEASE**.

This workflow runs only on pull requests that modify this exact workflow or `ci/dep01d/**`. It has `contents: read` permission, no repository writes, no signing credentials, no deployment, no self-hosted runner, no Holochain/admin/tunnel secret access, and no `workflow_dispatch` promotion path.

## Immutable inputs

- Repository: `Permaculture-DAO/prometheus-happ` (private)
- **Historical deployed source SHA:** `2e84d3a3f69ade0481cd5c778f98a9a68d82af6d` — **not** current main
- `Cargo.lock` SHA-256: `d92609a15c9ca9d37280cce74fd26f5fedc3268abfd92e0b16580bee42ef11a9`
- Rust/Cargo 1.96.0, `wasm32-unknown-unknown` target, release profile
- `.cargo/config.toml` **as it exists inside the source commit**: `getrandom_backend="unsupported"` and `-C link-arg=--allow-undefined`. No explicit `RUSTFLAGS` override.
- Existing deployed `zome_integrity.wasm` SHA-256: `3089b324b51c0d1e88c5c8e2652a60762be30d9c92a7f3b0ca8e57e032a9ce98`
- Existing deployed `zome_coordinator.wasm` SHA-256: `6b93a978f45fc1c4c8a9376fde9f605f84570a823d1154e15eff58b8db284370`

## Distinguish three types of evidence

1. **Prior local build** on federica: PASS for both WASM SHA-256 with **explicit custom backend** `RUSTFLAGS='--cfg getrandom_backend="custom" -C link-arg=--allow-undefined'`; documented in ops-docs draft PR #31.
2. **Repository-config build**: the config committed at the historical source SHA is `getrandom_backend="unsupported"`. Whether it independently yields equal bytes requires a separate check; it is NOT automatically covered by (1).
3. **Independent GitHub-hosted runner**: this workflow uses Ubuntu 24.04, installs pinned Rust 1.96.0, checks out the exact historical SHA and verifies `Cargo.lock`, then builds with the repository's own config. It fails closed if either produced WASM differs. CI success, once observed, would demonstrate a second build environment, **not independent human review or release signing**.

GitHub-hosted machines are an independent runtime from `federica`, but their secure provenance, executor image, supply chain and identity need independent review. A green check on one machine is not independently attested organization-level release authorization.

## Promotion blocked until

Codex source/build security review; Claude Genesis/Canon assurance review; authorized operator-signed provenance record and custody chain; independent clean-host/off-host restore evidence; reliable SLO; explicit steward adoption. No REAL data writes, market/financial decisions, TRBK or PRU rights, Canon changes, automatic merge or production deployment.

**Negative test interpretation:** build failure / checksum mismatch => HOLD. A lack of runner invocation or a blocked job => NOT_EXECUTED, not PASS. No file in this branch attests a signature.
