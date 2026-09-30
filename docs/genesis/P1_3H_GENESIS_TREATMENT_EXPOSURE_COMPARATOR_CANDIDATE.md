# PROMETHEUS P1.3H — Genesis Treatment Exposure Map & Comparator Candidate

Status: **CANDIDATE — DIRECT HYDRAULIC EXPOSURE FEATURES RESOLVED; INTERNAL CONTROL NOT YET ADMISSIBLE**  
Date: 2026-09-30

## 1. Purpose

Convert the verified local-metric vector design into an explicit treatment-exposure framework and test whether an internal contemporaneous untreated control can presently satisfy the P1.3D C1–C5 gate.

This document does not assert a treatment effect.

## 2. Source lineage

- irrigation project DXF: `progetto irriguo.dxf`
- SHA-256: `1c99461566ff015f61db8aa1f922c7d02e838a568a1356db1c2cd06c7b48222f`
- local design metric scale: 1 drawing unit treated as 1 metre under P1.3G QA;
- surveyed ExperimentalSystemBoundary remains A8D, 3360.875 m², and is not replaced by the design frame.

## 3. Direct hydraulic exposure features

The following source-vector lateral arrays are promoted as **direct hydraulic exposure evidence**, not as complete management-sector polygons.

| Exposure ID | Source layer | X extent (m) | Y extent (m) | source structure |
|---|---|---:|---:|---|
| D-HYD-NW-01 | PVC-DISTRIBUZIONE | 260.567041–268.767041 | 284.424750–297.624750 | 23 distinct lateral levels at 0.600 m spacing |
| D-HYD-SW-01 | PVC-DISTRIBUZIONE | 263.992702–273.592723 | 268.976558–279.776558 | 19 distinct laterals at 0.600 m spacing |
| D-HYD-ELOW-01 | PVC-DISTRIBUZIONE | 277.133297–295.557315 | 261.524750–269.924750 | 15 distinct lateral levels, with paired geometry |

These envelopes are derived from source LINE extents. They indicate direct irrigation exposure where linework is present. They do not assert that every point in the bounding envelope receives identical water.

## 4. Swale / hydrological vector corridor

Two source-vector curve families occupy the lower-field swale zone.

After 0.01 m coordinate snapping solely for topology reconstruction:

### ACQUA source family

- layer: `ACQUA - water hatch`
- principal merged curve length: approximately **128.946 m**
- bounds: X **266.56–285.01**, Y **229.97–256.53**

### Utilizzati source family

- layer: `Utilizzati`
- merged curve length: approximately **144.784 m**
- bounds: X **265.23–285.33**, Y **229.33–256.53**

These are source-vector curve families associated spatially with the designed swale/pond corridor.

**Important:** their merged lengths are not promoted as the swale centerline length. They may represent banks, parallel traces, outlines or composite curves. They therefore do not resolve the technical-report values of approximately 80 m, 65.8 m or 94.5 m.

The source-vector bounds are admissible for exposure screening; the exact swale centerline and influence buffer remain OPEN.

## 5. Treatment exposure classes

### T1 — Direct treatment/exposure

A location is T1 when source evidence shows direct:

- irrigation lateral or local irrigation infrastructure;
- swale/earthwork geometry;
- imported-soil placement where spatially documented;
- planting/bed construction;
- amendment/compost/mulch/cover-crop application;
- other focal regenerative intervention.

### T2 — Indirect influence

A location is T2 when direct treatment is absent but material influence may arise from:

- hydrological redistribution from the swale;
- irrigation wetting beyond the source line;
- runoff/convergence;
- roots/canopy/shading;
- nutrient movement;
- access/traffic;
- maintenance or management spillover.

No fixed T2 buffer distance is invented at this stage.

### C-candidate — potential internal comparator

A residual location may be labelled `C-candidate` only if it has no identified T1 exposure and can subsequently pass all C1–C5 gates.

### X — exclusion

Buildings, impermeable surfaces, access/infrastructure or other locations unsuitable for the tested endpoint are excluded from control/sampling as applicable.

## 6. Internal-control feasibility test — current result

### C1 sufficient area — **NOT YET PASSED**

The site may contain nominal residual land outside the seven reported irrigation-sector areas, but exact non-overlapping residual geometry has not yet been reconstructed. Area sufficiency cannot be inferred from arithmetic subtraction alone.

### C2 baseline comparability — **OPEN**

No candidate untreated polygon has yet been frozen and paired with baseline soil/topography/outcome measurements.

### C3 spillover isolation — **NOT YET PASSED**

The source vector confirms distributed irrigation and a substantial lower-field hydrological corridor. Exact T2 influence is not yet bounded. A residual visually outside an irrigation array cannot yet be declared isolated.

### C4 operational durability — **OPEN**

No polygon has yet been formally reserved as intervention-free for the preregistered comparison window.

### C5 agronomic/operational acceptability — **OPEN**

A reserve-control decision must be checked against the intended site design and required management.

### Gate result

**No internal contemporaneous control is currently admissible/frozen.**

This does not prove that an internal control is impossible. It means that no polygon presently satisfies the evidence required to call it one.

## 7. Comparator strategy consequence

Until an internal polygon passes C1–C5:

1. maintain repeated pre-intervention baseline measurements within the treatment system;
2. develop a matched external reference candidate in parallel;
3. do not make causal superiority claims from treatment-only before/after observations;
4. if a suitable internal residual zone is identified, freeze it before treatment exposure and before confirmatory outcome interpretation.

## 8. Required next step — P1.3I

Proceed to **P1.3I — Residual-Land Reconstruction & Control Reservation Decision**.

Required outputs:

1. map all source-backed T1 irrigation/earthwork/planting footprints;
2. map building/impermeable/access exclusions;
3. create a versioned residual-land geometry;
4. screen residual polygons against the swale/hydrological corridor and other T2 pathways;
5. rank residual polygons only by objective feasibility variables (area, separation, topographic/soil comparability), without calling any one a control until C1–C5 pass;
6. select/reserve an internal polygon if feasible, otherwise promote the external-reference pathway;
7. only after that freeze sampling coordinates and replication.

## 9. Claim boundary

P1.3H is an exposure-classification and comparator-feasibility record.

It does not establish ecological efficacy, causal effect, field validation, certification, legal cadastral geometry, PRU/TRBK value or capital-facing admissibility.
