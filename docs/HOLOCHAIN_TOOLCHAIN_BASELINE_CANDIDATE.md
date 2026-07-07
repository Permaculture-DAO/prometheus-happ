# Holochain toolchain baseline candidate — Sprint 0

Status: candidate; not a release decision.

The official Holochain 0.6 compatibility table currently recommends:

- Holochain conductor/core: `0.6.1`
- `hc` CLI: `0.6.1`
- HDK: `0.6.1`
- HDI: version compatible with the official 0.6 table
- Holonix branch: `main-0.6`

Before merge, the repository must record exact pins for all Holochain, Rust, Nix, Node and JavaScript client dependencies and update the lockfiles in one reviewed change.

The current `v1.1.3-runtime-proof` remains bounded synthetic evidence. A toolchain pin does not promote it to production.
