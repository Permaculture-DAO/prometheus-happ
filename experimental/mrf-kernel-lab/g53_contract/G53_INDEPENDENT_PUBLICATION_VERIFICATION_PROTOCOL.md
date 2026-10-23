# G5.3 — Independent Publication & Custodian Verification (TEST preflight)

**Status:** DRAFT. Physical gate HOLD_PENDING_INDEPENDENT_REVIEW. Mechanical verification is not evidence of independent people, machines or separately controlled external archives. No REAL, financial or token authority.

## Evidence required per real custodian

1. Independently verified organization, named authorized approving officer, key enrollment provenance, current key/revocation status, and separately administered infrastructure/failure domain. Independent reviewer must log the verification channel and time.
2. Distinct enrolled Ed25519 witness public-key fingerprint, verified through a trusted out-of-band channel controlled by THAT custodian, not by the coordinator. Never upload private keys, access tokens or passwords.
3. Same approved TEST session challenge: exact nonce, source digest, authorization, root epoch/history and latest witness receipt head. Verify independent nonce consumption and revocation records.
4. Custodian separately signs canonical publication statement with its own enrolled key. Signed statement binds publication raw-byte SHA-256, custodian ID, claimed publication channel, witness head/round, session nonce, positive publication sequence and predecessor head. Signed JSON is canonical UTF-8 and duplicate-key free.
5. Independent reviewer fetches signed publication records DIRECTLY from two separately authenticated and administratively distinct external publication services, verifies actual source origin, captures original bytes, corroborates freshness and pins the latest receipt head/sequence/predecessor outside the coordinator's trust domain. Copy-pasted URLs, documents, timestamp labels or public-key strings alone do not establish independent provenance.
6. Verify there are no observed conflicting publications for the same custodian and sequence and confirm continuity using independently stored previous heads, revocation history and receipt logs. Unknown unpublished equivocation and global replay remain unproven without a separately operating transparency register.

## Automated offline TEST checks

The new module g53_publication_signoff.py requires the entire validated G5.3 custody dossier plus two extra CANONICAL signed publication JSON files and separate per-custodian latest-head, predecessor-head and sequence pins. It checks signature under each custodian's existing witness public key using an explicit publication-only Ed25519 message domain, raw file canonicalization, exact claimed publication digest, session nonce, authorization-bound witness receipt, session timestamp window, channel and caller-provided head/sequence.

The verifier reruns the full synthetic historical root-key rotation, reviewer quorum, signed witness and terminal TEST SQLite checks. Its result can never imply real external retrieval, real independent ownership, global replay prevention or a GO. A claimed external head pinned by the same operator can be replaced with the signature/record: an independent external reviewer must authenticate the actual newest head.

Required output markers on local TEST PASS:

- G53_TEST_TWO_SIGNED_PUBLICATIONS=PASS
- G53_TEST_PER_CUSTODIAN_PUBLICATION_PINS=PASS
- G53_EXTERNAL_PUBLICATION_RETRIEVAL=NOT_PERFORMED
- G53_PUBLICATION_ORIGIN_AUTHENTICITY=NOT_PROVEN
- G53_CUSTODIAN_REAL_INDEPENDENCE=NOT_PROVEN
- G53_GLOBAL_PUBLICATION_ANTI_EQUIVOCATION=NOT_PROVEN
- G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW

## Physical admission blocker

Do NOT run physical Gate 5 or G5.4 until two custodians really enroll, independent reviewer authenticates direct publication evidence and current external pins and a second authorized physical host is online and verified under change control. Keep PR #54 draft. Do not merge or change Canon/REAL/DNA/production/token/finance boundaries.
