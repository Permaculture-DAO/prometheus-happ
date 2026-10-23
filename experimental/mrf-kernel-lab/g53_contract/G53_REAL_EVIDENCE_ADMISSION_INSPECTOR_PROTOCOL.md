# G5.3 — Physical Evidence Admission Inspection (offline TEST-only)

**Status:** draft preflight only. The external identity and authority of real custodians are NOT verified by this code. Never promote physical G5.3, G5.4, signed Canon, REAL persistence, token or financial controls from a filled-in register.

## Inspector contract

`g53_real_evidence_gate.py` reads `G53_EXTERNAL_EVIDENCE_REGISTER_v1.json` (currently 18 NOT_OBSERVED) and an expected SHA-256 over the canonical JSON from the **caller**. A digest must be independently approved and stored in a separate trust domain to serve as a genuine external pin. The CLI itself does not authenticate its origin.

Use from the G5.3 contract directory: `python3 g53_real_evidence_gate.py G53_EXTERNAL_EVIDENCE_REGISTER_v1.json --expected-sha256 <independently-approved-canonical-json-sha256>`

The inspector checks strict registry shape and the 18 ordered uniquely identified gates; explicitly forbids changing `automatic_go_permitted`, `unknown_is_not_proven`, signed Canon/production status or REAL persistence. It refuses an `NOT_OBSERVED` requirement bearing invented evidence, rejects a claimed review without a plain HTTPS original source, SHA-256 digest, observer identity reference, decision reference and exact UTC review/expiry timestamps; an expired claimed review is **not** documentary complete. No credentials, query strings or private keys belong in source URLs.

A subset may be recorded as EVIDENCE_SUBMITTED_UNVERIFIED with an original source and SHA, but without falsely asserting the independent reviewer has accepted it. For later separately reviewed evidence, the status `INDEPENDENT_REVIEW_CLAIMED` is **a self-reported documentary claim**, NEVER independently authenticated by the inspector. Other allowed statuses REJECTED and EXPIRED cannot be counted as positive. The inspector does not fetch the source, verify its signature, authenticate the alleged reviewer or prove the two real custodians are independent.

## Explicit adversarial contract

CI must reject forged source hashes, reordered/missing/duplicated gates, wrong project authority, `GO`, `OPEN`, extra promotion fields, unverified submissions falsely claiming approval, future review timestamps, insecure/credential-bearing links, missing evidence and stale claims. **Even 18/18 claimed reviews** can yield at most `CLAIMED_COMPLETE_UNAUTHENTICATED`, with `external_pin_provenance=NOT_AUTHENTICATED`, `independent_publication_retrieval=NOT_PERFORMED`, `automatic_go=false`, physical G5.3 HOLD and G5.4 HOLD.

A CLI exit status of zero means the **register schema was successfully inspected**, not that external evidence exists or that admission was approved. Nonzero exit 2 means the supplied register/digest is inconsistent or invalid. Independent human reviewer and competent authority must verify original sources directly and separately, document authority/consent, key enrollment and absence of common administrative and failure domains before an explicit, separately authorized physical TEST decision. `UNKNOWN != ZERO`; fail closed.

## Current external dependency

Two genuinely independent consenting custodians, two directly fetched independently pinned signed publications, a documented independent reviewer decision, and the second authorized physical host remain unobserved. **Physical G5.3 and G5.4 must remain HOLD** until these external prerequisites are satisfied. Do not contact candidates, launch physical tests, merge PRs or deploy anything merely because this inspector passed CI.