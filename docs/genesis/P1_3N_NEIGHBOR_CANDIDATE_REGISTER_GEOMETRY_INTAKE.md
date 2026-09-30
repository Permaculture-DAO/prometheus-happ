# PROMETHEUS P1.3N — Neighbor Candidate Register & Geometry Intake

Status: **ACTIVE INTAKE — ROUTE EXTERNAL SELECTED; CANDIDATE IDENTITIES/GEOMETRIES OPEN**  
Date: 2026-09-30

## 1. Purpose

Create the controlled intake layer for neighboring-land comparator candidates selected under P1.3L-EXT and operationalized by P1.3M.

No neighboring parcel, owner or geometry is invented at this stage.

## 2. Candidate register schema

Each candidate record SHALL contain:

- `candidate_id`
- owner/operator reference
- contact/access reference
- cadastral reference if supplied/verified
- polygon geometry reference
- CRS/state of coordinates
- usable area
- distance/direction from Genesis
- access status
- sampling permission status
- management-history summary
- current land use
- known recent amendments/earthworks/irrigation
- hydrological relationship to Genesis
- soil/profile evidence state
- topography evidence state
- vegetation/ground-cover baseline state
- expected management stability
- conflict-of-interest/disclosure notes
- evidence references
- candidate state: `OPEN | EXT-A | EXT-B | EXT-C | REJECTED`

## 3. Intake evidence classes

Candidate information must be tagged as one of:

- `OWNER_REPRESENTATION`
- `CADASTRAL_EVIDENCE`
- `SURVEY/GIS_EVIDENCE`
- `FIELD_OBSERVATION`
- `LAB_EVIDENCE`
- `MANAGEMENT_RECORD`
- `PHOTOGRAPHIC_EVIDENCE`
- `OTHER_VERIFIED_SOURCE`

Owner statements remain useful evidence but are not silently upgraded to measured field facts.

## 4. Geometry intake rule

A candidate may enter screening with a locator point, but cannot enter comparator freeze until a closed polygon is available.

Required geometry state:

1. stable candidate ID;
2. closed polygon;
3. declared coordinate system or explicit `CRS_UNRESOLVED`;
4. area;
5. provenance/source;
6. geometry version/hash;
7. relationship to Genesis boundary;
8. excluded subareas where known.

A hand-drawn or screenshot-derived polygon must be labeled `DERIVED` until registered against verified control points.

## 5. Minimum owner/access intake

Before any sampling:

- owner/operator identified;
- permission to enter documented;
- permission to collect soil/water/vegetation observations documented;
- permission to remove soil samples documented where applicable;
- expected access window;
- restrictions;
- management-change notification expectation;
- contact/reference record.

No land right or scientific equivalence is inferred from access permission.

## 6. Fast rejection screen

Reject or downgrade a candidate if any of the following is material and cannot be controlled:

- recent major soil import/export;
- recent deep earthworks;
- markedly different parent material;
- materially different slope/hydrologic position;
- direct Genesis runoff/irrigation influence;
- unstable or unknown near-term management;
- inaccessible for repeated measurements;
- owner unwilling to disclose material interventions;
- insufficient usable area for the intended sampling frame.

## 7. Candidate intake table

| Candidate ID | Location/parcel | Geometry | Access | Management stability | Baseline comparability | State |
|---|---|---|---|---|---|---|
| EXT-001 | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN |
| EXT-002 | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN |
| EXT-003 | OPEN | OPEN | OPEN | OPEN | OPEN | OPEN |

The initial placeholders are register slots only; they do not represent identified parcels.

## 8. Selection discipline

Candidate registration, eligibility screening and baseline matching must occur before treatment-outcome inspection.

A candidate cannot be introduced after the outcome is known merely because it yields a favorable contrast.

## 9. Immediate field intake packet

For each candidate collect, before endpoint baseline:

- locator/polygon source;
- access authorization;
- current photos from fixed directions if feasible;
- basic land-use history;
- current irrigation/fertilization/tillage status;
- recent earthworks or imported soil;
- visible drainage/runoff relationship;
- slope/aspect/topographic notes;
- vegetation/ground-cover state;
- proposed sampling-safe zone.

## 10. Promotion gate to P1.3O

P1.3N can move forward only when at least one real external candidate has:

- identifiable location;
- geometry source;
- access state;
- management-history information;
- no obvious disqualifying spillover;
- sufficient evidence to enter baseline matching.

Until then, exact comparator geometry and confirmatory sampling coordinates remain OPEN.

## 11. Claim boundary

P1.3N is an intake/register layer only. It does not establish equivalence, causal effect, field validation, certification, PRU/TRBK value or capital-facing admissibility.
