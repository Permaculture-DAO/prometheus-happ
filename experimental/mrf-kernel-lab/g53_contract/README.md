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

## Session-bound Ed25519 attestations (additional TEST gate)

The optional standalone G5.3 tool, g53_attestation.py, cryptographically checks two signed, canonicalized host manifest claims against a short-lived challenge. It uses OpenSSL Ed25519 verification and never reads private signing keys. Unit-test keys are generated **ephemerally** on the CI runner and are not host identities or release credentials.

Call with the same policy and host manifests checked by g53_preflight.py, plus a challenge, independently provisioned public keys and binary (64-byte) Ed25519 signatures:

    python3 g53_attestation.py \
      --policy /secure/policy.json --host-a /secure/a.json --host-b /secure/b.json \
      --challenge /secure/challenge.json --approved-challenge-sha256 DIGEST_FROM_SEPARATE_APPROVAL \
      --pub-a /secure/a.pub.pem --pub-b /secure/b.pub.pem \
      --sig-a /secure/a.sig --sig-b /secure/b.sig

The challenge JSON must contain exactly these fields: schema=PRM-G53-ATTESTATION-CHALLENGE-v1, nonce_hex (32 random nonzero bytes in lowercase hex), issued_at_epoch, expires_at_epoch, approved_test_source_sha, and host_a_pubkey_sha256/host_b_pubkey_sha256. The public-key digests use DER SubjectPublicKeyInfo bytes, **not** PEM text. Both signing agents sign the domain-separated bytes defined by g53_attestation.signed_message(challenge, role, manifest), incorporating the hashes of the **complete canonical challenge and role-specific manifest**. A 30-minute maximum validity window is enforced. Challenge SHA-256 is calculated over sorted-key compact UTF-8 JSON; it must be approved **out of band** and supplied to the verifier.

Passing the signature check proves only control of keys matching the supplied pinned public-key digests at signing time. It **does not** prove independently enrolled/trusted keys, tamper-free physical telemetry, independent physical failure domains or unique nonce usage. Identical signed payloads can be replayed **within** a valid challenge window unless an independent approval service tracks nonce consumption. The tool therefore outputs G53_TRUSTED_KEY_ENROLLMENT=NOT_PROVEN, G53_NONCE_NONREUSE=NOT_PROVEN, G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN, and G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW even when cryptographic checks PASS.

Unit tests use fabricated manifests and temporary Ed25519 keys, including mutated claims, swapped signatures, changed challenge, stale/future/oversized validity windows, wrong trust pins and REAL boundary negatives. This is a proof-of-possession regression suite, not G5.3 acceptance.

## G5.3 — Offline witness assertion and supplied nonce-ledger snapshot (TEST only)

Use g53_witness_audit.py only with an approved, independently authenticated **reviewer public-key fingerprint**, two signed host TEST manifest claims, and an approved challenge digest. The separate witness registry binds challenge SHA-256, both host signer-key fingerprints, exact source SHA, approval reference and a **third distinct Ed25519 reviewer public key**. The reviewer signs PRM-G53-REVIEW-v1 domain-separated bytes over the canonical registry digest. In CI, all three key pairs are temporary fictional credentials.

The strict nonce-ledger snapshot JSON is schema PRM-G53-NONCE-SNAPSHOT-v1 with a prior_consumptions list. Every entry has exactly nonce_hex and challenge_sha256. The audit rejects duplicates within the supplied snapshot, a repeated candidate nonce, or a repeated candidate challenge digest. It never appends or edits the ledger or contacts a registry; an omitted/truncated/forged snapshot cannot be detected merely by this verifier. Max 128 previous entries; this is a **bounded audit input**, not a production-grade long-lived ledger.

A passing check prints G53_TEST_REVIEWER_ED25519_SIGNATURE=PASS and G53_NONCE_UNUSED_IN_SUPPLIED_SNAPSHOT=PASS but also G53_REVIEWER_KEY_ENROLLMENT=NOT_PROVEN, G53_NONCE_LOG_COMPLETENESS=NOT_PROVEN, G53_ATOMIC_NONCE_CONSUMPTION=NOT_PROVEN and G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW. Absence in a user-supplied log cannot establish global nonce nonreuse or replay protection under concurrency. A separate authenticated trust registry and transactional nonce-consumption store with concurrency control are required before any physical G5.3 GO.

This is test-only, not a release signature, not REAL persistence, not live Holochain authority and not a scientific or financial certification.


## G5.3 — Transactional nonce-claim laboratory (one local TEST SQLite file)

Two TEST-only modules, g53_nonce_store.py and test_g53_nonce_store.py, extend the bounded audit. Before any write, the verifier **recomputes the complete Ed25519 host and reviewer checks** on caller-supplied synthetic fixtures. On success it attempts to claim the session challenge SHA-256 and nonce via SQLite BEGIN IMMEDIATE, with UNIQUE constraints on both keys. A separate explicit provisioning function creates the ephemeral store; the claim path refuses missing stores. A clock check is repeated inside the transaction.

SQLite uses synchronous=FULL, immutable UPDATE/DELETE triggers and a schema check that refuses missing uniqueness/trigger guards. Each concurrent client gets its own connection; tests include six simultaneous contenders, a separate Python process attempting the same insert, replay after reopening the database, changed signed challenge with a reused nonce, distinct fresh signed nonce, bad signature/REAL-class rejection before consumption, and missing/corrupted database schema.

**Limit:** this proves atomic behavior only among contenders accessing the SAME local TEST database. It does NOT establish distributed nonce uniqueness, trusted reviewer enrollment, physical host identity, a production-grade highly available service, cross-host consensus, secure backups, or partition resilience. An administrator who can replace or roll back the file can defeat the SQLite-internal append-only triggers. No production services, live conductors, private credentials or real data are involved.

The success path explicitly retains G53_DISTRIBUTED_NONCE_CONSENSUS=NOT_PROVEN, G53_LEDGER_OPERATOR_TRUST=NOT_PROVEN, G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN and G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW. No changes to production, Canon, tokens, finances or REAL_DATA_PERSISTENCE_GATE_CLOSED are authorized.
