# Gate 5.3 — Independent-host TEST preflight contract

**Status: manifest schema candidate, NOT physical-host PASS.** The controlling [Gate-5 DHT contract](../GATE5_DHT_REPLICATION_TEST_CONTRACT_2026-10-21.md) requires independently initialized agents, conductor state and verified DHT provenance. Same-runner G5.2 success does not prove host independence.

## Read-only structural check

An authorized operator must separately create three JSON files: a policy and a manifest from each physical TEST host (role A/B). The policy contains the reviewed source commit SHA, independently approved minimum disk and memory availability in bytes, and an approval reference. **There are no hidden or default resource thresholds.**

Each host manifest must contain precisely: schema PRM-G53-PREFLIGHT-v1; role A or B; test source SHA; the four pinned DNA/WASM/Cargo.lock SHA-256 values; opaque SHA-256 identifiers of the host, agent public key and state root; failure-domain and observation references; free_disk_bytes and available_ram_bytes; and booleans production_runtime_touched=false and real_persistence_enabled=false.

Validate by running (from this directory):

    python3 g53_preflight.py --policy path/to/policy.json --host-a path/to/a.json --host-b path/to/b.json

The linter makes **no network calls, no conductor operations, no key access and no modifications**. A conforming *fabricated* manifest produces G53_MANIFEST_PAIR_SCHEMA=PASS but always G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN, G53_DHT_REPLICATION=NOT_TESTED and G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW. It rejects unknown fields, mismatched pinned hashes, identical claimed identities/domains, missing observations, inadequate claimed resources and open production/REAL boundaries.

Do not publish signing secrets, service credentials, raw device identifiers or production state paths. Hashed labels and claims can be fabricated: they **cannot substitute for independent witness review**.

## Conditions to lift G5.3 HOLD

Independent reviewers must authenticate the two actual host evidence sources, resources and physical failure-domain separation; approve isolated TEST-only network routes, distinct conductor stores and a revert plan; verify actual cryptographic artifact digests directly from each machine; then perform bounded A→B and B→A nonproduction tests with author-signed ActionHash, typed receipt, DHT integration and cache-origin evidence. GetOptions::default warm reads do **not** themselves demonstrate a remote network RTT or global DHT validation.

G5.4 additionally requires a controlled partition, bounded UNKNOWN/FAIL attribution, rejoin and verified convergence. A pass here is never ecological certification, capital authority, token rights or permission to open REAL_DATA_PERSISTENCE_GATE_CLOSED. Do not boot production WSL, merge Canon or replace the signed production DNA as part of these checks.

The included 15 unit tests use **only fabricated TEST host data**, proving negative control behavior, not host attestations.
## JSON file-level negative controls

The CI executes both in-memory schema tests and **the actual command-line entrypoint** on fabricated files. JSON objects containing duplicate member names at any level or non-finite numbers are rejected explicitly. Missing, oversized (>32 KiB), malformed, inconsistent and production/REAL-open files return exit code 2. A structurally valid *fabricated* pair returns process status 0 for **schema validation only**, while still printing `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW` and `G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN`; do **not** equate an exit code of 0 with permission to run a physical experiment. External reviewer sign-off and host observation remain mandatory.
