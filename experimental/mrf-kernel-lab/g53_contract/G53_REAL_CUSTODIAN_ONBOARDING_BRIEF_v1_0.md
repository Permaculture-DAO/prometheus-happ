# PROMETHEUS — G5.3 Real Custodian Onboarding Brief v1.0

**2026-10-23 · Controlled candidate-engagement brief · TEST-only · PR #54 · Not ratified or legally binding**

## Purpose, source and limits

Request TWO separately administered external custodians of the same bounded **TEST-only** G5.3 witness challenge; authenticate operator identity and independently controlled key custody before any physical-host claim. This brief is a request for evidence and prospective engagement, **not a request for anyone to sign real production sensor, token, financial, scientific, valuation or legal assertions**. No approval or delivery is claimed.

Applicable governance boundary: PROMETHEUS_AUTHORITY_LATTICE_v1_0.yaml, `F_FACTUAL` direct observation / competent measurement / independent verification over self-asserted derived models; `I_IMPLEMENTATION` verified repo/runtime state over narrative; invariants `AI_IS_NON_AUTHORITATIVE`, `TRANSPORT_DOES_NOT_CREATE_AUTHORITY`, `UNKNOWN != ZERO`. The Source Root Manifest (PRM-ROOT-SOURCES-001) separates constitutional, implementation, legal and factual authority. This software repo and this brief cannot confer or supersede such authority.

Latest baseline under review: draft PR https://github.com/Permaculture-DAO/prometheus-happ/pull/54 . Evidence is accepted only against one explicitly fixed TEST source SHA, approved authorization reference, DNA/WASM/lock digests and unique challenge from the appointed coordinator. Do not create/submit a real challenge before independent reviewer confirmation.

## Requested custodians

| Role | Required actual evidence | Present status |
|---|---|---|
| Custodian A | Independently identified responsible organization/person, signed service scope, authorized official and trusted contact path | **NOT_OBSERVED** |
| Custodian B | Same, with distinct real accountable organization, administrative control, key custody, publication infrastructure and failure domain | **NOT_OBSERVED** |
| Independent reviewer | Named authorized reviewer **separate from the TEST coordinator**, documented authority, controlled verification workflow and decision record | **NOT_OBSERVED** |
| Independent host B | Real independently administered host identity, remote attestation or equivalent trusted inventory, isolated conductor environment and network/failure-domain map | **NOT_OBSERVED** |

Do not infer separate ownership merely from different email addresses, IPs, cloud account labels, machine names or public-key fingerprints. An accredited/qualified identity provider may support enrollment, but verify actual authority and separation by independently authenticating the approval route. Verify actual permissions applicable to the jurisdiction and engagement; no contractual rights are created by repository labels.

## Candidate intake — evidence required from EACH custodian

1. **Authority / identity:** official organization name and registration/identity source; actual signing authority; conflict-of-interest declaration; independently corroborated identity-validation record. Keep personal IDs and private personal data out of public GitHub logs.
2. **Key lifecycle:** Ed25519 public SPKI DER SHA-256, proof the custodian can sign a *reviewer-issued* nonce, evidence of independent private-key control, key creation/activation and recovery custody, suspension/revocation contact and key compromise response. Never send private keys, seed phrases, credentials or tokens.
3. **Signed fixed TEST claim:** same domain-separated history commitment (root-policy and reviewer epochs, source SHA-256, latest witness head and previous witness head, authorization, short validity window, nonce). Retain actual signed raw bytes, not screenshots or paraphrases.
4. **Publication:** separately hosted append-only/immutable or auditable archive publication; exact signed-publication byte digest, controlled publication endpoint, latest signed head, sequence, predecessor, independently corroborated time and any available inclusion/consistency proof. The independent reviewer must retrieve each endpoint **directly**, without using files merely re-uploaded by the coordinator.
5. **Independence:** owner/admin/operations contacts, administrative domains, signing key operators, hosting accounts, physical/failure-domain mapping, backups, time sources, change-management controls, audit logs and separation evidence. Assess common billing/admin roots or shared single points of failure; labels alone are insufficient.
6. **Review and cancellation:** written authorization to carry out the bounded TEST exercise, withdrawal path, privacy/security handling plan, revocation lookup and retained append-only evidence timeline; any unresolvable conflict, stale certificate/key or missing record is HOLD.

## Candidate external anchoring technologies (NOT named custodians)

- Sigstore Rekor is a potential **signature-transparency log**. Current official overview: https://docs.sigstore.dev/logging/overview/ . Require independently validated inclusion and consistency evidence; posting to the same public log twice does **not** create two independent custodian organizations. Rekor inclusion is not identity verification or guarantee of factual truth.
- OpenTimestamps is a potential **timestamp/commitment anchor**. Official description: https://opentimestamps.org/ . A verifiable timestamp demonstrates existence by a given time, not signer identity, complete history, independent administration or ongoing real-time availability. It is not a substitute for independently controlled custodian publication channels.
- RFC 3161 TSA is an alternative for corroborated time evidence (https://www.rfc-editor.org/info/rfc3161/). Verify the time-stamp token's signature, time and certificate/revocation context; a TSA alone is not a G5.3 custodian.

These are architectural **options to evaluate**, not configured or contacted services. Provider availability, terms, accepted artifact formats and trust-root pinning must be checked before use.

## Candidate request — copyable scope

PROMETHEUS seeks an independent, bounded TEST-only evidence custodian for a pre-production G5.3 two-host witness audit. Please provide your verified institutional identity and approval contact, independent Ed25519 public key enrollment and custody controls, a separately administered retrieval/publication channel with immutable receipt history, security and revocation procedures, and conflict-of-interest/operational independence disclosures. No token, investment, asset value, scientific certification, production persistence or legal signing right is requested. A reviewer independent of the TEST coordinator will confirm your identity and agree a controlled test challenge before any signature or connection.

## Gate transition and stop criteria

**Current G5.3 physical = HOLD** until BOTH custodians and the independent reviewer are authentically enrolled and directly observed, independently pinned witness/publication heads are available, physical host B is authorized and reachable, and a separate project authority gives written narrow TEST approval under the active authority lattice. Unknown is NOT a zero-risk or passed condition. No merge, real deployment, G5.4 partition/rejoin or change to REAL_DATA_PERSISTENCE_GATE_CLOSED is authorized by this document.
