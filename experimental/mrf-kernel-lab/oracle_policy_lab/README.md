# MRF Gate 2 — Oracle Independence Shadow Lab

NON-AUTHORITATIVE, **TEST data only**. Pure Rust, no conductor integration or field data.

Verifies actual Ed25519 signatures for domain-separated, fixed-format `Attestation` bytes with versioned trusted signer registry. It applies frozen-time freshness, revocation, no repeated physical sample digest, no same-device duplicate and **distinct independent operator groups and methods**. Outcomes: `Agreement`, `Disagreement`, `Unknown`, `RequiresReview`. Agreement is **a bounded agreement among registered, cryptographically verified signers**, not biological truth, scientific assurance, DHT consensus, certification, funding authority, insurance status or token rights.

A simple ecological sentinel checks four signed-integer indicators for missing fields and worsening soil, water or biodiversity and increasing external inputs. Its best outcome is `CandidateOnly`, NEVER `VERIFIED`. Real data need site-specific measurement units, calibration, uncertainty, independent MRV, reference plots and governance review.

Attack tests: correlated signers under same operator, forged/altered Ed25519 claims, physical sample reuse, stale/future data, signer revocation, unknown signer, missing calibration and contradictory labs. Trusted *operator group* assignments are configuration facts requiring separate offline authority validation. The verifier **cannot detect undisclosed ownership collusion or fraudulent physical sampling**; cryptography authenticates bytes, not the environment.

No source modification, no Holochain zome merge, no key generation/storage workflow, no REAL gate opening. Issue #42 tracks separate source/device/field evaluation and Reticulum/edge benchmarking.

Run: `cargo test --manifest-path experimental/mrf-kernel-lab/oracle_policy_lab/Cargo.toml`.
