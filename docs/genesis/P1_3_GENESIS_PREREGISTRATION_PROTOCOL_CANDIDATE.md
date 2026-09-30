# PROMETHEUS P1.3 — Genesis Pilot Preregistration Protocol Candidate

Status: **CANDIDATE / SITE IDENTITY RESOLVED, BOUNDARY GEOMETRY NOT YET FROZEN**  
Date: 2026-09-30  
Data/claim discipline: evidence-led, falsifiable, auditable.

## 1. Site identity — supplied evidence

User-supplied cadastral map capture identifies the intended Genesis Pilot parcel as:

- Country: Italy
- Region: Sicily
- Province: Ragusa (RG)
- Comune: Ragusa (RG)
- Foglio di mappa: **250**
- Particella: **714**
- Point displayed within the selected parcel: **36.79569195273318, 14.550539006229448**
- Cadastral-map source shown in capture: https://mappecatasto.it/c1.htm

This resolves the **parcel identity** for preregistration purposes. The displayed point is a locator within the selected cadastral parcel; it is **not** treated as a polygon or legal boundary geometry.

## 2. Terrain/topography evidence — supplied capture

A second user-supplied capture from Contour Map Creator, used within the Permaculture Design workflow, shows a rectangular sampling extent over the intended pilot area:

- source: https://contourmapcreator.urgr8.ch/
- workflow context supplied by user: https://permaculturedesign.io/your-dream
- displayed contour range: approximately **77–91 m**
- displayed number of levels: **18**
- sampling grid: **20 × 20**
- displayed endpoint fields:
  - “North West corner”: latitude approximately **36.7962**, longitude approximately **14.5513**
  - “South East corner”: latitude approximately **36.7953**, longitude approximately **14.5498**

The endpoint labels/longitude ordering as displayed should not be silently normalized: the screenshot is retained as source evidence, but these values require export/verification before machine-readable geometry is frozen.

The contour rectangle is **terrain sampling evidence**, not proof of the cadastral boundary and not yet the canonical SystemBoundary.

## 3. Boundary state

| Boundary element | State | Rule |
|---|---|---|
| cadastral identity | RESOLVED | Ragusa, foglio 250, particella 714 |
| locator point | RESOLVED | point evidence only |
| legal cadastral polygon | OPEN | require authoritative/vector boundary or verified vertex coordinates |
| experimental treatment polygon | OPEN | must be explicitly frozen before field baseline |
| comparator/reference polygon | OPEN | must be spatially explicit and preregistered |
| contour sampling rectangle | EVIDENCE_ONLY | cannot substitute for legal or experimental boundary |

No polygon is inferred from a screenshot.

## 4. Preregistered scientific architecture

### 4.1 Primary question

For the bounded Sicily Genesis site, do preregistered regenerative/syntropic interventions produce measurable changes in selected ecological and system-state outcomes relative to the preregistered baseline and comparator/reference, under a fixed observation calendar and explicit disturbance/missing-data rules?

This is a testable research question, not a claim that regeneration has occurred.

### 4.2 Hypotheses and estimands

Before baseline collection, each primary outcome MUST define:

1. measurement variable and unit;
2. treatment spatial unit;
3. comparator/reference spatial unit;
4. baseline window;
5. follow-up window(s);
6. estimand (change, difference-in-change, ratio, or other prespecified quantity);
7. effect direction, if directional;
8. uncertainty reporting, including confidence interval;
9. exclusion/missing-data rule;
10. falsification/non-support criterion.

No post-hoc outcome may be presented as preregistered.

### 4.3 Intervention ledger

Every intervention MUST be time-stamped and spatially bound, including at minimum:

- intervention class;
- date/time;
- operator/source;
- affected geometry or sampling unit;
- material/input where relevant;
- intensity/quantity where measurable;
- linked evidence;
- deviations from protocol.

Intervention recording does not establish ecological efficacy.

### 4.4 Baseline gate

Field-effect claims remain blocked until the baseline package is complete for the frozen treatment and comparator/reference boundaries.

Baseline completion requires:

- boundary freeze;
- sampling design freeze;
- instrument/method specification;
- observation calendar;
- primary-outcome definitions;
- baseline observations;
- provenance and quality flags;
- disturbance/adverse-event ledger initialized.

### 4.5 Comparator/reference gate

A comparator/reference MUST be selected before outcome evaluation. Its geometry, selection rationale, management history where known, and material differences from the treatment area must be recorded.

Absence of a defensible comparator/reference must be surfaced as a limitation and may restrict causal language.

### 4.6 Missing data and disturbances

Missing observations are never silently imputed. Each missing datum receives a reason/status when known. Disturbances, sensor failures, management deviations, extreme weather, contamination, access failures, and protocol deviations are recorded in an append-only disturbance/adverse-event ledger.

Sensitivity analyses must be identified before capital-facing claims where missingness or disturbances could materially alter interpretation.

### 4.7 Review and COI

Independent review requires:

- reviewer identity;
- scope;
- declared conflicts;
- independence-for-scope attestation;
- review date;
- evidence package hash/reference;
- finding and limitations.

A conflicted reviewer cannot satisfy an independent-pass gate for the same scope.

## 5. Evidence Spine binding

The preregistration maps to the existing semantic model:

`PlaceContext → SystemBoundary → Observation → EvidencePackage → ReviewAttestation → AdmissibilityDecision`

and may bind interventions/disturbances and semantic claim references as applicable.

The runtime authority boundary remains:

`admissibility_only_no_value`

Recorded ≠ verified ≠ admissible ≠ valuable ≠ investable.

## 6. Claim gates

Until field evidence and independent review satisfy the preregistered gates:

- ecological truth: **NOT ESTABLISHED**
- scientific validation: **NOT ESTABLISHED**
- certification: **NOT ESTABLISHED**
- legal admission: **NOT ESTABLISHED**
- market admission: **NOT ESTABLISHED**
- PRU/TRBK value: **NOT ESTABLISHED**
- Sicily Genesis field validation: **NOT ESTABLISHED**

Cryptographic integrity does not prove physical truth.

## 7. Required facts before P1.3 site-complete freeze

The following remain blocking:

1. authoritative/verified cadastral polygon or vertex set for foglio 250, particella 714;
2. exact treatment polygon if it differs from the full parcel;
3. exact comparator/reference polygon;
4. baseline start date and observation calendar;
5. primary outcome set, methods, units, instruments and sampling locations;
6. intervention start date and intervention plan;
7. reviewer/review-scope assignment;
8. protocol deviation and adverse-event handling owner.

## 8. Promotion rule

P1.3 may move from **CANDIDATE** to **SITE-COMPLETE PREREGISTRATION** only when all blocking spatial and methodological facts above are frozen, internally consistent, and hashable.

P1.3 site-complete status is still not field validation. Subsequent evidence must be collected and reviewed under the frozen protocol before claims are promoted.
