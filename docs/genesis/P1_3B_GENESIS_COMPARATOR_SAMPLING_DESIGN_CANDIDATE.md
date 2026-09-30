# PROMETHEUS P1.3B — Genesis Comparator & Sampling Design Candidate

Status: **CANDIDATE — DESIGN FROZEN IN PRINCIPLE; COMPARATOR GEOMETRY + FIELD LOCATIONS OPEN**  
Date: 2026-09-30

## 1. Inputs already frozen

- Site: Ragusa (RG), foglio 250, particella 714.
- Surveyed Genesis ExperimentalSystemBoundary: DXF outline A8D.
- Surveyed area: **3360.875 m²**.
- Surveyed perimeter: **257.426 m**.
- Geometry: irregular 13-vertex polygon.
- Terrain evidence supplied separately indicates an elevation gradient across the site; contour capture is contextual evidence, not canonical boundary geometry.

## 2. Experimental design principle

The comparator must not be selected merely because it is convenient or visually adjacent. It must be selected to minimize systematic differences from the treated area that could explain observed outcomes independently of the Genesis intervention.

Accordingly, comparator selection must consider, where measurable:

- elevation/topographic position;
- slope and aspect;
- soil/parent-material context;
- prior land management and disturbance history;
- vegetation/cover state;
- hydrologic position;
- exposure/access effects;
- baseline values of primary outcomes.

No causal claim is permitted solely from treatment-before/after observations without a defensible reference strategy.

## 3. Preferred comparator hierarchy

### A. Internal contemporaneous control — preferred when operationally feasible

Reserve one or more spatially explicit units inside the surveyed boundary that do **not** receive the focal Genesis intervention during the primary comparison window.

Advantages: strongest control of broad site context.

Constraint: buffer against treatment spillover, hydrologic coupling, shading/root influence, edge effects, traffic and management contamination.

### B. Matched external reference — required fallback / complementary reference

Select an external area with documented matching criteria and freeze its polygon before outcome evaluation.

It must not be called equivalent unless baseline evidence supports comparability.

### C. Historical baseline — supportive, not a substitute by default

Repeated pre-intervention measurements within the treatment units strengthen inference but do not automatically replace a contemporaneous comparator.

## 4. Spatial stratification

Because topographic heterogeneity can confound ecological outcomes, sampling must be stratified rather than treated as one homogeneous 3360.875 m² unit.

Before field sampling, derive or verify terrain strata from geospatial evidence. At minimum assess:

- upper / middle / lower topographic position;
- slope classes;
- aspect/exposure where meaningful;
- drainage/convergence zones;
- edges and anthropogenic disturbance zones.

The contour screenshot must not be used to manufacture exact DEM values. Prefer exported elevation data, survey points, or another traceable elevation source for the canonical terrain layer.

## 5. Sampling-unit rules

Sampling locations MUST be frozen before treatment-effect evaluation.

Each point/plot receives a stable identifier and records:

- SystemBoundary ID;
- stratum;
- treatment/control status;
- coordinates in a declared CRS;
- measurement method/instrument;
- observation schedule;
- disturbance flags;
- evidence/provenance reference.

Sampling density is **not yet numerically fixed** because the primary outcomes, spatial variance, measurement methods and feasible plot/point dimensions have not yet been frozen. A power/precision rationale or defensible pilot-design rationale is required before finalizing sample count.

## 6. Buffer and exclusion policy

Predefine exclusions rather than removing inconvenient observations later. Candidate exclusion classes include:

- boundary/edge buffer;
- buildings or impermeable surfaces;
- access/traffic corridors;
- areas materially altered before baseline;
- sensor-invalid locations;
- zones where treatment/control spillover cannot be bounded.

Every exclusion requires a spatial record and reason.

## 7. Repeated-measures calendar

Preferred structure:

1. pre-intervention baseline window with repeated observations where feasible;
2. intervention timestamp/ledger;
3. fixed post-intervention observation windows;
4. event-triggered observations only when separately labelled;
5. long-term follow-up windows retained where outcome dynamics require them.

Calendar dates remain OPEN until intervention timing and primary outcomes are frozen.

## 8. Analysis skeleton

For each primary outcome preregister:

- outcome definition/unit;
- spatial unit;
- baseline statistic;
- follow-up statistic;
- treatment contrast;
- comparator contrast;
- estimand;
- effect size;
- uncertainty interval;
- missing-data treatment;
- sensitivity analysis;
- falsification/non-support rule.

Where the design supports it, a change-in-treatment relative to change-in-comparator estimand may be used. The statistical model must be selected before unblinded outcome interpretation and must reflect repeated/spatial structure where relevant.

## 9. Evidence Spine mapping

Candidate chain:

`PlaceContext → ExperimentalSystemBoundary → SamplingUnit/Observation → EvidencePackage → ReviewAttestation → AdmissibilityDecision`

Comparator/reference geometry must be linked to the same PlaceContext while remaining explicitly typed as comparator/reference rather than treatment.

## 10. Blocking facts for P1.3B freeze

The following are still required:

1. decision on internal control availability and intervention-free duration;
2. exact comparator/reference polygon(s);
3. canonical terrain/elevation dataset or verified survey elevation points;
4. primary outcome set;
5. measurement methods/instruments;
6. baseline and intervention dates;
7. sample-count/precision rationale;
8. field sampling coordinates;
9. reviewer assignment and COI declaration.

Until these are frozen, this document is a design candidate, not a completed experimental preregistration.

## 11. Claim boundary

This design does not establish ecological efficacy, scientific validation, certification, legal/market admission or PRU/TRBK value. It establishes only the candidate architecture required to test future claims under bounded, auditable conditions.
