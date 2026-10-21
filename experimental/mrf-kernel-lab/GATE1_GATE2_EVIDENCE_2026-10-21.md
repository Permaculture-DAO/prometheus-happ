# MRF Gate 1/2 interim verification — 2026-10-21

Authority: TEST_ONLY / SHADOW / noncanonical. This is a *subordinate experimental branch* based on draft MRF PR #39, NOT a new controlling source or approved release.

## Verified on PC Federica / WSL
- Gate 1: 10/10 HDK 0.6.1 / HDI 0.7.1 Rust host unit tests passed. The new validation callback SHA-256 hashes the **serialized baked synthetic model bytes** and checks against the admitted model digest before checking the receipt claim. This closes the previous *string-comparison-only* gap for this exact frozen toy model.
- Gate 1 new integrity-zome WASM release build: PASS. WASM SHA-256 `44830822ed1cc510d024c209c99403d41cca7e2238a9594d03e58f46a87fd23a`. The coordinator was unchanged, SHA-256 `dc86b0ab8f48d3c88bd1385a2cc248e46115630999a9614f534bca63dbee12bb`.
- New TEST-only packed DNA: SHA-256 `b1df1cfe3a6825d7e2af61ca5772b7cb762251d53f23e62f0f7bf9a6b05fd793`. **This differs from predecessor** `8f60c6b9e9fe9d3722720376a5aa30ca49720268a2a8fdbd036f35f61aba6efc`. Prior predecessor conductor PASS is not proof for the new DNA.
- Gate 2: 8/8 synthetic policy tests passed locally. Important: `signature_verified`, `fresh`, `calibration` flags are **assumed test fixtures**, not verified external facts or cryptographic signatures. Distinct identity/operator labels alone cannot prove actual independence or custody.

## Still OPEN
- Exact new-DNA conductor tests must PASS; record output and duration (not implied by packaging).
- Tests for *incorrect baked model bytes with unchanged declared digest* must be built and executed (the host test currently checks baked-model digest equality, not a compiled alternate model).
- Holochain FlatOp coverage: StoreRecord/RegisterUpdate/RegisterDelete and multi-peer validations, missing dependencies.
- Real Ed25519 signatures over canonical attestation payload; revocation/freshness/replay; independent operator-control registry.
- Oracle causal/physical measurement corroboration, ecological invariants, Goodhart and multi-site scientific MRV.
- Independent DNA rebuild and audit/signed governance review.

No production/REAL/code path/Canon/PRU/RAP/TRBK/financial authority.
