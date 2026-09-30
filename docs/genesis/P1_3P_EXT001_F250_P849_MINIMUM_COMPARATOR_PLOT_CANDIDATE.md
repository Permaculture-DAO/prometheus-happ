# PROMETHEUS P1.3P — EXT-001 / Foglio 250 Particella 849 Minimum Comparator Plot Candidate

Status: **CANDIDATE — PARCEL IDENTITY RESOLVED; MINIMUM COMPARATOR FRAME DERIVED; CADASTRAL POLYGON / ACCESS / BASELINE MATCHING OPEN**  
Date: 2026-09-30

## 1. Candidate identity

User-supplied cadastral-map capture identifies the first external comparator candidate as:

- Comune: **Ragusa (RG)**
- Foglio di mappa: **250**
- Particella: **849**
- locator point displayed by the source: **36.79627423238698, 14.55050988807528**
- source shown in capture: MappeCatasto.it
- screenshot SHA-256: `bbdb31adf1cfccd8024baa9957fcafbc89bfa8a4aff0b838c3a07685ba69d69a`

The point is treated as a parcel locator only. It is not the legal cadastral polygon.

Candidate identifier:

`EXT-001-F250-P849`

## 2. Minimum useful comparator surface

For the current pilot stage, the minimum useful spatial frame is set as:

- **gross candidate frame: 30 m × 30 m = 900 m²**
- **internal analysis core: 20 m × 20 m = 400 m²**
- **nominal internal edge buffer: 5 m on each side**

This is a **minimum spatial-design frame**, not a statistical power conclusion.

### Rationale

The 400 m² core is large enough to host a stratified field sampling layout for infiltration, aggregate stability, SOC and companion covariates without placing all measurements immediately on parcel/management edges.

The surrounding 5 m internal buffer separates confirmatory sampling from the gross plot boundary and leaves room for disturbance/access screening.

Final sample count remains controlled by baseline variance, spatial autocorrelation, measurement footprint and the precision/power rationale. The existence of 400 m² does not by itself make observations independent.

## 3. Provisional derived geometry

Pending an authoritative/vector cadastral polygon, a **DERIVED** 30 × 30 m square has been constructed around the supplied locator point solely as a candidate field frame.

Coordinate state: **WGS84 / EPSG:4326, geodesically derived from the supplied locator**.

Approximate gross-frame corners:

| corner | latitude | longitude |
|---|---:|---:|
| NW | 36.79640939927191 | 14.55034181788665 |
| NE | 36.79640939927191 | 14.55067795826391 |
| SE | 36.79613906526148 | 14.550677957673305 |
| SW | 36.79613906526148 | 14.550341818477257 |

Approximate 20 × 20 m analysis-core corners:

| corner | latitude | longitude |
|---|---:|---:|
| NW | 36.796364343670334 | 14.55039784134849 |
| NE | 36.796364343670334 | 14.550621934802072 |
| SE | 36.79618412099671 | 14.550621934539588 |
| SW | 36.79618412099671 | 14.550397841610973 |

The geometry is not source-equivalent to the cadastral polygon. Before comparator freeze, it must be checked in field/CAD/GIS against the actual parcel boundary and owner-access area.

## 4. Why 900 m² gross / 400 m² core is the minimum selected

A smaller gross plot would rapidly lose usable area after:

- parcel/management-edge exclusion;
- access/disturbance exclusion;
- local hydrological anomalies;
- minimum separation among sampling points;
- destructive soil sampling footprints;
- repeated-measure revisit requirements.

A 30 × 30 m frame preserves a 20 × 20 m interior core after a 5 m internal buffer while remaining small relative to the visible extent of Particella 849.

This is deliberately conservative but field-practical.

## 5. Preliminary sampling architecture

Until baseline variance is available, P1.3P does **not** freeze confirmatory sample count.

Candidate feasibility architecture:

- 12–16 spatially distributed infiltration locations within the 400 m² core;
- soil aggregate-stability samples co-located or paired under a frozen SOP;
- SOC sampling using the final composite/independent-point rule selected in P1.3C/D;
- companion soil-moisture and ground-cover observations;
- all points versioned before confirmatory analysis.

These counts are feasibility targets only and must not be represented as powered confirmatory replication.

## 6. EXT-001 eligibility state

| Criterion | Current state |
|---|---|
| parcel identity | **RESOLVED** |
| locator point | **RESOLVED** |
| authoritative cadastral polygon | **OPEN** |
| 900 m² candidate frame | **DERIVED / PROVISIONAL** |
| owner/access permission | **OPEN** |
| management history | **OPEN** |
| hydrological independence from Genesis | **OPEN** |
| soil/parent-material comparability | **OPEN** |
| slope/topographic comparability | **OPEN** |
| vegetation/ground-cover baseline | **OPEN** |
| management stability | **OPEN** |
| comparator class EXT-A/B/C | **OPEN** |

## 7. Admission rule

EXT-001 may enter baseline qualification once:

1. the 30 × 30 m frame is confirmed to lie inside the usable Particella 849 area;
2. access and soil-sampling permission are documented;
3. recent management/earthworks/irrigation history is recorded;
4. no material Genesis spillover is identified;
5. the same baseline SOPs can be applied on Genesis and EXT-001.

It cannot be classified EXT-A or EXT-B before baseline matching.

## 8. Next roadmap node — P1.3Q

Proceed to **P1.3Q — EXT-001 Field Eligibility & Baseline Matching Intake**.

Required inputs:

- landowner/operator permission;
- current and recent land management;
- irrigation/fertilization/tillage state;
- known soil import/export or earthworks;
- field confirmation of the candidate plot location;
- basic topographic/hydrological screening;
- first matched baseline measurements.

## 9. Claim boundary

Particella 849 is now an identified external comparator candidate, not a validated equivalent control.

The 900 m² frame and 400 m² core are methodological constructs. They do not establish ecological efficacy, causal effect, field validation, certification, PRU/TRBK value or capital-facing admissibility.
