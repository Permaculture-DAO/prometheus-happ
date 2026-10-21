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


## Final local follow-on results — 2026-10-21

**PASS — exact NEW SHA-bound DNA live conductor** (not predecessor):
```
b1df1cfe3a6825d7e2af61ca5772b7cb762251d53f23e62f0f7bf9a6b05fd793  prometheus_mrf_synthetic_lab.dna
running 1 test
GATE_1_TEST_VALID=PASS
GATE_2_OUTPUT_FORGERY_REJECTED=PASS
GATE_3_REAL_REJECTED=PASS
GATE_4_MODEL_SUBSTITUTION_REJECTED=PASS
GATE_5_FINANCIAL_PROMOTION_REJECTED=PASS
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 125.10s
```
Execution: an isolated WSL `/home/giova/prometheus-mrf-gate1-20261021` laboratory with a copy of the new signed-*neither*-release TEST DNA, copied frozen conductor harness and cached build dependencies. No production conductor accessed; test wall runtime including build ≈212.56s. The tests assert five behavior scenarios in one in-process conductor, **not five independently owned peers**.

**PASS — Ed25519 synthetic cryptographic unit tests (4/4)** in `tests/oracle_ed25519_synthetic.rs`: valid signature, tampered value/sequence/expiry/other public key rejected, length-delimited framing prevents ambiguity, exact-repeat in-memory replay guard, and expiry remains a separate policy condition. Keys are synthetic fixed-byte test keys; no network key registry, no real independent operators, no witnessed measurements, no real custody or revocation verification. Do **not** infer scientific corroboration or production readiness.

**PASS — oracle independence policy checks (8/8)**, synthetic only, distinct from Ed25519. True source/operator independence is not demonstrated by a text label.

**Current status:** Gate 1 *bounded in-process conformity and actual-model hash binding* PASS. Gate 1 *all Holochain op variants, independent multi-peer tests and full signed DNA reproducibility* OPEN. Gate 2 *synthetic signature primitive and policy checks* PASS, full trusted oracle validation/ecological anti-gaming OPEN. REAL remains CLOSED.
