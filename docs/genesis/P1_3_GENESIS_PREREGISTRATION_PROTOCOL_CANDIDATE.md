# PROMETHEUS P1.3 — Genesis Pilot Preregistration Protocol Candidate

Status: **CANDIDATE / SITE IDENTITY + SURVEYED EXPERIMENTAL SYSTEM BOUNDARY FROZEN**  
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

## 3. Survey geometry — DXF extraction

The user supplied `RILIEVO TERRENO.dxf`, converted from the original survey DWG. Direct DXF inspection provides an explicit surveyed site outline and drawing annotations.

Drawing units are metres (`$INSUNITS = 6`). The survey drawing explicitly annotates:

- `sup. catastale mq 3594,00`
- `sup. reale mq 3.360,87`
- `diff.-mq 233,13`

The site-outline polyline (DXF handle `A8D`) contains 13 distinct vertices plus a repeated closing vertex. Independent shoelace calculation from the DXF coordinates gives **3360.875 m²**, consistent with the drawing annotation `sup. reale mq 3.360,87`.

Calculated surveyed perimeter: **257.426 m**.

Ordered surveyed vertices (drawing coordinates, metres):

| Vertex | X | Y | next segment (m) |
|---|---:|---:|---:|
| V01 | 2479861.738 | 4072309.266 | 12.637 |
| V02 | 2479853.607 | 4072318.940 | 24.429 |
| V03 | 2479838.307 | 4072337.985 | 25.143 |
| V04 | 2479857.287 | 4072354.474 | 17.390 |
| V05 | 2479870.442 | 4072365.848 | 26.501 |
| V06 | 2479890.392 | 4072383.292 | 22.871 |
| V07 | 2479907.727 | 4072398.211 | 16.095 |
| V08 | 2479918.022 | 4072385.839 | 19.884 |
| V09 | 2479930.881 | 4072370.673 | 15.626 |
| V10 | 2479919.184 | 4072360.312 | 19.950 |
| V11 | 2479904.215 | 4072347.123 | 29.787 |
| V12 | 2479881.871 | 4072327.426 | 12.984 |
| V13 | 2479872.230 | 4072318.730 | 14.129 |

The repeated final DXF vertex equals V01 and closes the outline.

### Geometry interpretation

This DXF resolves the **surveyed physical site geometry** and its measured area. It does not, by itself, prove that the surveyed outline is legally identical to the cadastral polygon. The drawing itself distinguishes `sup. catastale` from `sup. reale` and includes the annotation `confine ipotetico catastale`; therefore the cadastral/legal boundary remains a separate evidence class.

For the Genesis experiment, the user has confirmed that the entire surveyed DXF outline is the Genesis Pilot area. The DXF outline A8D is therefore frozen as the canonical physical `ExperimentalSystemBoundary` for the pilot (3360.875 m²; perimeter 257.426 m). It must not be silently relabelled as the authoritative cadastral polygon.

## 4. Boundary state

| Boundary element | State | Rule |
|---|---|---|
| cadastral identity | RESOLVED | Ragusa, foglio 250, particella 714 |
| locator point | RESOLVED | point evidence only |
| surveyed physical-site polygon | RESOLVED | DXF outline A8D; 3360.875 m²; 257.426 m perimeter |
| legal cadastral polygon | OPEN | cadastral identity resolved, but legal/vector boundary still requires authoritative evidence |
| experimental treatment polygon | FROZEN | full surveyed DXF outline A8D; 3360.875 m²; user confirmed entire surveyed polygon is the Genesis Pilot area |
| comparator/reference polygon | OPEN | must be spatially explicit and preregistered |
| contour sampling rectangle | EVIDENCE_ONLY | cannot substitute for legal or experimental boundary |

No polygon is inferred from a screenshot.

## 5. Preregistered scientific architecture

### 5.1 Primary question

For the bounded Sicily Genesis site, do preregistered regenerative/syntropic interventions produce measurable changes in selected ecological and system-state outcomes relative to the preregistered baseline and comparator/reference, under a fixed observation calendar and explicit disturbance/missing-data rules?

This is a testable research question, not a claim that regeneration has occurred.

### 5.2 Hypotheses and estimands

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

### 5.3 Intervention ledger

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

### 5.4 Baseline gate

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

### 5.5 Comparator/reference gate

A comparator/reference MUST be selected before outcome evaluation. Its geometry, selection rationale, management history where known, and material differences from the treatment area must be recorded.

Absence of a defensible comparator/reference must be surfaced as a limitation and may restrict causal language.

### 5.6 Missing data and disturbances

Missing observations are never silently imputed. Each missing datum receives a reason/status when known. Disturbances, sensor failures, management deviations, extreme weather, contamination, access failures, and protocol deviations are recorded in an append-only disturbance/adverse-event ledger.

Sensitivity analyses must be identified before capital-facing claims where missingness or disturbances could materially alter interpretation.

### 5.7 Review and COI

Independent review requires:

- reviewer identity;
- scope;
- declared conflicts;
- independence-for-scope attestation;
- review date;
- evidence package hash/reference;
- finding and limitations.

A conflicted reviewer cannot satisfy an independent-pass gate for the same scope.

## 6. Evidence Spine binding

The preregistration maps to the existing semantic model:

`PlaceContext → SystemBoundary → Observation → EvidencePackage → ReviewAttestation → AdmissibilityDecision`

and may bind interventions/disturbances and semantic claim references as applicable.

The runtime authority boundary remains:

`admissibility_only_no_value`

Recorded ≠ verified ≠ admissible ≠ valuable ≠ investable.

## 7. Claim gates

Until field evidence and independent review satisfy the preregistered gates:

- ecological truth: **NOT ESTABLISHED**
- scientific validation: **NOT ESTABLISHED**
- certification: **NOT ESTABLISHED**
- legal admission: **NOT ESTABLISHED**
- market admission: **NOT ESTABLISHED**
- PRU/TRBK value: **NOT ESTABLISHED**
- Sicily Genesis field validation: **NOT ESTABLISHED**

Cryptographic integrity does not prove physical truth.

## 8. Required facts before P1.3 site-complete freeze

The following remain blocking:

1. authoritative/verified cadastral polygon or explicit acceptance that cadastral geometry remains a separate unresolved legal evidence layer;
2. exact comparator/reference polygon;
4. baseline start date and observation calendar;
5. primary outcome set, methods, units, instruments and sampling locations;
6. intervention start date and intervention plan;
7. reviewer/review-scope assignment;
8. protocol deviation and adverse-event handling owner.

## 9. Promotion rule

P1.3 may move from **CANDIDATE** to **SITE-COMPLETE PREREGISTRATION** only when all blocking spatial and methodological facts above are frozen, internally consistent, and hashable.

P1.3 site-complete status is still not field validation. Subsequent evidence must be collected and reviewed under the frozen protocol before claims are promoted.
