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


## G5.3 — Read-only, externally pinned TEST ledger checkpoint (anti-rollback detector)

The additional g53_ledger_checkpoint.py computes a **canonical content and schema digest** from a read-only SQLite connection to the one-host TEST nonce store. It checks SQLite integrity, the exact expected columns and primary key, nonce uniqueness, presence of update/delete guards, and well-formed existing rows. Observations contain the reviewed TEST source SHA-256, total row count, a domain-separated digest of ordered rows, and a domain-separated digest of ledger table/trigger schema. No database creation, claim, transaction replay, network call, secrets or host service startup occurs.

To verify, an **independent custodian** must previously approve and retain the exact SHA-256 of the canonical checkpoint JSON *outside the SQLite database and outside the authority of its file operator*. The verifier is then passed that checkpoint JSON, its independently pinned digest and reviewed source SHA-256; it fails closed if the present TEST database has fewer/more claims, different row content, a modified guard/schema, an inconsistent source, or a forged checkpoint/digest combination.

CI performs only **fabricated checkpoint and rollback experiments**, including restored older files, unchanged-count row alteration by a privileged TEST fixture, direct unapproved insert, guard removal and incorrect external pins. These prove conditional detection **assuming the externally supplied pin is authentic and current**. An attacker with power to replace both the database and its supposed pin, or to show the reviewer an earlier legitimate pin, can still defeat this design. The checker does NOT establish an independently signed latest checkpoint, public append-only log, distributed rollback prevention, external-key enrollment, cross-host nonce consensus, hardware identity or partition behavior.

Checkpoint comparison is a read-only audit. It cannot stop a rollback that occurs between verifications and does not replace an atomic consensus-backed claim service. All success paths remain explicitly `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW`. The PR is draft; signed production DNA/Canon, REAL persistence, finance and TRBK/PRU/RAP remain unchanged.


## G5.3 — Domain-separated signed checkpoint witnesses (synthetic only)

g53_checkpoint_witness.py verifies an Ed25519 checkpoint witness statement with **exactly**: schema PRM-G53-CHECKPOINT-WITNESS-v1, positive `sequence`, `previous_head_sha256`, complete G5.3 `checkpoint`, `approved_test_source_sha256`, `issued_at_epoch`, `witness_pubkey_spki_sha256`, and a TEST authorization reference. The signed message is the domain-separated SHA-256 of the canonical witness statement; the content-addressed witness head is SHA-256 of that message. It is not a production certificate.

The read-only verifier refuses the statement unless it matches an **out-of-band, reviewer-controlled** latest-head SHA-256, latest sequence number, predecessor-head SHA-256, independently reviewed source SHA and Ed25519 witness public-key SPKI digest. Sequence 1 requires an all-zero predecessor; a higher sequence requires a nonzero predecessor equal to the external prior-head pin. Signature verification occurs before the existing checkpoint/SQLite state comparison. Future-dated witness claims beyond an explicit 60-second allowance are rejected.

The GitHub CI suite generates only ephemeral Ed25519 keys and fabricated local SQLite receipts. It tests valid chains, an older signed attestation replayed after newer sequence, mutated signed checkpoint with recalculated head, a rollback of the local SQLite file, same-row-count tampering, wrong predecessor and reviewer key, bad signature, absent store, invalid sequence, and unrelated negative controls. All tests must be proven by the exact CI run before recording a PASS.

**Scope:** Even a signature matching the pinned key is NOT proof that the reviewer is independent or enrolled, and a caller who substitutes **both** the latest-head pin and the signed checkpoint may still present an older valid state. A previously committed *independently authenticated* latest head is essential. The checker cannot tell if its alleged external head is genuinely the newest, cannot prevent rollback between checks, does not run multi-host consensus, and cannot establish physical-host independence or G5.4 partition recovery. Evidence outputs retain `G53_AUTHENTIC_OUT_OF_BAND_LATEST_HEAD=NOT_PROVEN`, `G53_INDEPENDENT_WITNESS_KEY_ENROLLMENT=NOT_PROVEN`, and `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW`.

No Holochain DNA changes, no production services, no Canon merge and no opening of `REAL_DATA_PERSISTENCE_GATE_CLOSED` are permitted by this laboratory.


## G5.3 — Full TEST checkpoint history with multi-reviewer quorum

g53_checkpoint_quorum.py verifies a bounded history of 1–32 TEST checkpoint statements. Each statement binds strict schema, positive 1-based sequence, predecessor signed-record SHA-256, complete local SQLite checkpoint, approved TEST source SHA-256, approval reference and timestamp. Each signed record includes the statement and sorted unique Ed25519 endorsements. Every entry requires at least two distinct reviewers drawn from a sorted registry of 2–8 reviewer key fingerprints.

Verification requires caller-supplied *out-of-band-approved* SHA-256 of the reviewer registry, PEM public keys matching its DER SPKI fingerprints, externally pinned latest history head SHA-256 and latest sequence, and approved source/authorization references. Each intermediate signed-record head must match the next predecessor and sequence values must be gapless. Individual row counts and times may not regress. The terminal checkpoint is verified against the present SQLite TEST database; historical checkpoints are only signed historical claims, not independently recreated database states.

The associated new TEST unit suite creates three ephemeral reviewer key pairs and exercises 2-of-3 and 3-of-3 signatures, duplicate/missing/bad endorsements, substitutions of signer keys, unsigned/altered content, incomplete/reshuffled history, old valid history replay, a re-signed alternative fork, database rollback, regression, false external pins and schema/source failures. No real reviewer identity is enrolled; no Holochain or host service is contacted.

**Nonclaims:** A claimed external reviewer registry/latest head can be forged or rolled back if not actually authenticated and independently retained; multiple signatures from temporary test keys do not imply independent people, failure domains or governance. This verifier does not provide a distributed consensus service, independent historical-data re-execution, cross-host nonce uniqueness, physical-host evidence, G5.4 partition recovery, ecological validity or financial/token authority. `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW` remains mandatory on the successful path. No production DNA, Canon or REAL gate is changed.


## G5.3 — Root-signed TEST reviewer enrollment, revocation and latest-head anchoring

`g53_root_enrollment.py` is a standalone, **read-only** wrapper around the existing full-chain quorum checkpoint verifier. It requires a strict `PRM-G53-ENROLLMENT-ANCHOR-v1` envelope binding the reviewed TEST source SHA-256, authorization reference, SHA-256 of the **active** reviewer registry, revoked reviewer fingerprints, latest approved checkpoint head and sequence, incrementing policy epoch, predecessor policy head, and 30-minute maximum validity window. A root endorsement signs domain-separated canonical anchor bytes using Ed25519. At least two distinct root signatures are required; the root public-key fingerprints and claimed latest anchor head/epoch/predecessor are **separate externally pinned inputs**.

The wrapper checks the real public-key DER SPKI fingerprints, distinct root signers, disjoint root/reviewer key sets, sorted unique revocations with no revoked member in the active reviewer registry, expiry, source/authorization constraints, pinned previous anchor and latest anchor. It then delegates full-history multi-reviewer signature and terminal TEST SQLite checkpoint verification to `g53_checkpoint_quorum.py`, using ONLY values authorized in the root-signed envelope as the reviewer registry hash and latest checkpoint head/sequence.

The synthetic fixture generates three ephemeral root keypairs and three separate reviewer keypairs entirely on CI. Test cases include successful 2-of-3 and 3-of-3 signatures, policy epoch 2 linked to epoch 1, old signed anchor replay, substituted root key, single/duplicate or corrupted root signatures, forged registry/latest head, expired/oversized policy window, revoked reviewer erroneously left active, a new root-signed active roster excluding a revoked reviewer, missing DB and source/authorization boundary controls.

**Strict limits:** ephemeral root-key signatures do not authenticate actual people, organizations or independent failure domains. External root-key enrollment, approved policy epoch, current anchor-head freshness and its previous anchor still need *real, separately operated custodians*; if a single party controls both the TEST DB and all root trust pins, a valid old policy can still be replayed. The wrapper does not verify a complete historical root-policy epoch chain, key compromise recovery, hardware identities, quorum split-brain, cross-host nonce consumption, or partition recovery. All outputs retain `G53_EXTERNAL_TRUST_ANCHOR_PROVENANCE=NOT_PROVEN`, `G53_LATEST_ANCHOR_FRESHNESS_PROVENANCE=NOT_PROVEN`, `G53_ROOT_KEY_REVOCATION_AND_ROTATION=NOT_TESTED`, and `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW`.

No root credentials are committed; no live Holochain, federica/WSL, production Canon, runtime, REAL gate or TRBK/finance actions occur.


## G5.3 — Historical TEST checkpoint signature validity across policy epochs

`g53_epoch_history.py` audits a strictly bounded chain of TEST reviewer-policy epochs, preserving original historical checkpoint signatures. A sequence of up to eight root-signed policies authorizes a different active reviewer registry in each epoch. Each policy is signed by a quorum of distinct, caller-pinned Ed25519 root keys; predecessor policy hashes and the terminal policy-head SHA-256/epoch must match out-of-band references. The reviewer-key registry of each epoch is locked to that specific signed root-policy envelope.

Within each epoch, checkpoint records are checked for contiguous global sequences, predecessor-head SHA-256 linkage, reviewer signatures under ONLY the registry active at the time, non-decreasing row counts, and timestamps inside the policy validity window. Cumulative revocations may not be removed and a revoked key cannot reappear in a later active roster. **Historical signatures are left intact**, including those from keys subsequently revoked. The first policy and checkpoint history are anchored at zero-hash genesis. Only the final checkpoint is compared read-only to the current TEST SQLite store; historical states are not independently reconstructed.

The 24 new TEST-only fixtures use three ephemeral reviewer keys and three separate ephemeral root keys, two sequential policies with a reviewer revoked between them, old signature preservation, old-policy replay, missing/reordered history, attempts to use revoked signers in a later epoch, policy/record forks, root and reviewer signature tampering, rotated roster mismatch, revoked-key re-enrollment, old DB state, wrong external latest-head pin, chronological errors, and expiry of the latest policy. All original test keys and DBs remain isolated to CI.

**Boundaries:** root signing keys are held in the same CI runner; no independent human reviewer, root custody, certified hardware clock, external immutable store, distributed consensus, historical database reconstruction or two-host failure-domain separation is verified. The verifier explicitly emits `G53_EXTERNAL_ROOT_PIN_AUTHENTICITY=NOT_PROVEN`, `G53_ACTUAL_REVIEWER_INDEPENDENCE=NOT_PROVEN`, `G53_DISTRIBUTED_CONSENSUS=NOT_PROVEN`, `G53_PHYSICAL_INDEPENDENCE=NOT_PROVEN`, and `G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW`. Root-key rotation itself is not demonstrated; this milestone tests review-key rotation under a fixed root set.

No production DNA, Canon, token/financial authority, or `REAL_DATA_PERSISTENCE_GATE_CLOSED` is changed.
