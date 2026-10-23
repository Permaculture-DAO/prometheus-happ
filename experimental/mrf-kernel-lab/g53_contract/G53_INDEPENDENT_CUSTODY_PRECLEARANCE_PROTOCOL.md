# G5.3 Independent Custody Preclearance — controlled TEST protocol (draft)

**Authority:** TEST-only evidence-collection protocol within draft PR #54. No change to Canon, signed DNA/WASM, production runtime, finance/token rights or REAL_DATA_PERSISTENCE_GATE_CLOSED.

## Roles and evidence

1. TEST coordinator records exact source commit SHA, pinned TEST DNA/WASM/lock digests, bounded session nonce, test source hash and authorization reference. The coordinator does not independently approve its own evidence.
2. Custodian A and Custodian B must be different authorized human/organizational operators, with separately controlled signing keys and separately administered publication channels. Distinct labels, PEM files or CI-generated keys alone never demonstrate real separation.
3. An independent reviewer personally authenticates each custodian identity and Ed25519 public key fingerprint through an out-of-band channel, checks authority, suspension/revocation and control-plane/failure-domain separation. Private signing keys and credentials must NOT be uploaded into GitHub, CI or messages.
4. Each custodian independently receives the same signed TEST challenge/history commitment, signs it with its own key, publishes the same witness receipt head, and retains an immutable or append-only, separately controlled record of that signed evidence and its independently corroborated timestamp.
5. The reviewer receives each publication *directly through separately authenticated channels*, pins the independently received latest witness head and records a signed disposition. Source URLs or local paths provided solely by the coordinator are not proof of publication.

## Offline automated checks

The TEST-only g53_custody_preclearance.py checks a strictly pinned G5.3_TEST_ONLY dossier and two local evidence pairs (enrollment plus publication) against the actual validated TEST root-handover/witness receipt. It confirms that byte digests, public-key fingerprints, session nonce, source, authorization, epoch, latest witness head and publication references agree. The packet must declare distinct organization, administrator, physical failure-domain, custody, approval and publication references; these remain UNVERIFIED CLAIMS until independently examined.

A positive offline check is evidence-packet completeness only, and MUST print:
- G53_TEST_CUSTODY_PACKET_BYTE_BINDINGS=PASS
- G53_REAL_CUSTODIAN_IDENTITY_AUTHENTICATION=NOT_PROVEN
- G53_INDEPENDENT_ADMIN_AND_FAILURE_DOMAINS=NOT_PROVEN
- G53_REAL_OUT_OF_BAND_PUBLICATION_PROVENANCE=NOT_PROVEN
- G53_EXTERNAL_DOSSIER_PIN_AUTHENTICITY=NOT_PROVEN
- G53_PHYSICAL_CROSS_HOST_REPLICATION=NOT_TESTED
- G53_GO_NO_GO=HOLD_PENDING_INDEPENDENT_REVIEW

A coordinator can fabricate matching evidence files, identities, separate-looking labels, fresh timestamps and two temporary key pairs on one CI host. The offline module therefore does NOT establish independent custody, legal identity, actual external publication, real timestamps, global nonce uniqueness or hardware separation.

## Independent review required before physical G5.3

The approval record must identify actual reviewer, review method, identity verification source, authorized key fingerprints, separate storage/control planes, two independently published same-head receipt references, freshness and key revocation check, and complete TEST source/host attestation pins. A designated project authority must issue a separate GO decision under the authority lattice; this TEST checker never issues GO.

If any item is missing, unverifiable, or appears to be controlled by one party, remain HOLD. If the second real host is offline or not independently authorized, remain HOLD. Do not substitute two conductors on one CI runner for physical host independence. G5.4 network partition/rejoin stays HOLD until physical G5.3 admission.

No merge, Canon change, REAL persistence, token/financial issuance or production conductor action is permitted by this dossier.
