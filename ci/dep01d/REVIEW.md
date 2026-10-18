# DEP-01D — Independent runner gate for deployed WASM (review-only)

Status: **DRAFT / NOT MERGED / NOT A SIGNED RELEASE**.

This PR extends the **existing** `.github/workflows/ci.yml` `happ-ci` pipeline with the narrow `dep01d-pinned-wasm` job. The existing pipeline already runs on pull requests. The new job has `contents: read` permission, no repository writes, no signing credentials, no deployment, no self-hosted runner, no Holochain/admin/tunnel secret access, and no promotion path. The initially proposed separate DEP-01D workflow was removed from this branch to avoid competing pipelines.

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
2. **Repository-config build**: the config committed at the historical source SHA is `getrandom_backend="unsupported"`; a second clean local build **successfully** reproduced both deployed WASMs with no RUSTFLAGS override. The independent GitHub-hosted runner **also reproduced both bytes** after the historical Cargo registry source-path pin, as documented in the verified Actions run below.
3. **Independent GitHub-hosted runner**: the new job in the existing workflow uses Ubuntu 24.04, installs pinned Rust 1.96.0, checks out the exact historical SHA and verifies `Cargo.lock`, then builds with the repository's own config. It fails closed if either produced WASM differs. CI success, once observed, would demonstrate a second build environment, **not independent human review or release signing**.

GitHub-hosted machines are an independent runtime from `federica`, but their secure provenance, executor image, supply chain and identity need independent review. A green check on one machine is not independently attested organization-level release authorization.

## Promotion blocked until

Codex source/build security review; Claude Genesis/Canon assurance review; authorized operator-signed provenance record and custody chain; independent clean-host/off-host restore evidence; reliable SLO; explicit steward adoption. No REAL data writes, market/financial decisions, TRBK or PRU rights, Canon changes, automatic merge or production deployment.

**Negative test interpretation:** build failure / checksum mismatch => HOLD. A lack of runner invocation or a blocked job => NOT_EXECUTED, not PASS. No file in this branch attests a signature.


## Completed audit controls (2026-10-18)

- Exact-head [GitHub Actions run 39107223386](https://github.com/Permaculture-DAO/prometheus-happ/actions/runs/39107223386): independent WASM SHA-256 checks for both zomes PASS, ordinary build-test PASS, secret-scan PASS, gateway-ci PASS.
- Workflow-wide `permissions: contents: read` and `persist-credentials: false` on ordinary checkout introduced at commit `6b9b79f1bf20003b02958a99fd483d1e4c4599c3`. Both prior functional/build checks remained green.
- Linked non-authoritative technical findings and unresolved release-signature gates: [ops-docs draft PR #32](https://github.com/Permaculture-DAO/prometheus-ops-docs/pull/32), `ops/release_approval/DEP02A_SECURITY_PREAUDIT_2026-10-18.md`.
- Still **HOLD**: independent human signed supply-chain review, pinned trusted runner/toolchain attestation, root-of-trust governance, approved release decision, off-host DR, longitudinal SLO and TRL9. No merge or deployment.
