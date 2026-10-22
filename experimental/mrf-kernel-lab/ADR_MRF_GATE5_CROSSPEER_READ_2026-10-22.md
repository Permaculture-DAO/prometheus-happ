# ADR-MRF-014 — TEST-only network lookup of an immutable receipt (Gate 5 candidate)
Date: 2026-10-22 · Status: **DRAFT / LAB ONLY** · Source: PRM-SRC-MRF-ZOME-EDGE-001 candidate · Parent: [PR #48](https://github.com/Permaculture-DAO/prometheus-happ/pull/48)

## Scope and invariant
The source-controlled hApp `experimental/mrf-kernel-lab` includes a new **read-only** coordinator function `get_synthetic_receipt_for_audit(ActionHash)`. It calls HDK 0.6.1's `get(action_hash, GetOptions::network())`, then binds the returned record to the same requested action hash, requires a decodable `MrfSyntheticReceipt`, and returns a structured result for audit. Decode errors use an explicit `WasmError` rather than silently becoming empty records.

No real-world evidence, no token/financing/certification admission, no production DNA write, no return-path elevation from a read. Holochain DHT read success would only establish retrieval of a record in the bounded network, **not** scientific truth, independent signer custody, full distributed authority consensus or causal ecology.

## Verified local compilation and artifact identity (2026-10-22)
- `cargo check --offline` on the new coordinator: **PASS** after fixing explicit conversion from `SerializedBytesError` to `WasmError`.
- `cargo build --offline --release --target wasm32-unknown-unknown`: **PASS**.
- New TEST-only coordinator WASM SHA-256: `0e4b6e2531536e7b33f41f3806c284d1569cc805dcf870cf50fa4eff821a59b1`.
- Unchanged Gate-4 synthetic integrity-zome WASM SHA-256: `6cefdc096c665a7662fc48351800d94be31d5ad8d4450efce03cd4c9bdf61c93`.
- Independently packed **NEW synthetic TEST DNA** SHA-256: `1d20d26ec6d03af81e3cc7f85906615c4894c65bc843b6f949f969afb851d009`.
- The prior Gate-4 DNA SHA-256 `8f32a4bf0ca75f04649671cc6d5b73cecebdb0bd36a51fb58700f3837717abca` remains unchanged and is NOT evidence for the new DNA.

## Falsifiable remote-lookup test
A and B initialize separately with the same exact new TEST DNA and independent local source chains. After peer-info exchange, A creates **one** immutable valid TEST synthetic receipt and records its ActionHash. B does **not** author a receipt: B invokes the new network lookup endpoint with *A's ActionHash*. A conforming response must bind the exact action hash and all critical fields, including TEST namespace, model digest, integer vector, and `financial_admission=false`.

Any missing value, wrong hash/fields, decoding error, timeout, network partition or failed full test must be recorded as FAILURE/UNKNOWN, not as a scientific admission or PASS. The test is bounded and does not silently claim peer authority consensus.

## Known upstream observations and open gates
- On old Gate-4 DNA, one-conductor seven scenarios PASS; two-conductor **peer-info discovery** PASS.
- On old Gate-4 DNA, A→B sequential writes timed out after successful A CREATE; isolated B CREATE **PASS**; reversed B→A sequential writes **PASS**, 202.70s, one test, zero failures. This disproves a categorical claim that the second writer is always blocked, but does not prove why A→B stalled.
- New Gate-5 crosspeer test execution, multi-authority op validation, independent CI full-DNA reproduction, DHT partition/rejoin, oracle provenance and ecological MRV remain **separate gates**. Tests must run to completion before being claimed.
- The laptop's system drive was found to have <1 GiB and later ~9 GiB free; D remains <1 GiB. Use isolated tests and monitor physical disk safety. Never touch production Prometheus-Holochain WSL or runtime.

**No-go:** `REAL_DATA_PERSISTENCE_GATE_CLOSED` unchanged; no PR merge to signed main without code/security/steward review.
