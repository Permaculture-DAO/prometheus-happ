# PROMETHEUS P1.3J — Genesis Comparator Geometry Freeze Workpack

Status: **ACTIVE WORKPACK — INTERNAL ROUTE UNFROZEN; EXTERNAL REFERENCE FALLBACK ACTIVATED IN PARALLEL**  
Date: 2026-09-30

## 1. Gate inherited from P1.3I

P1.3I established that gross area does not rule out an internal comparator:

- agricultural area described as unaltered by building works: **2993.3 m²**;
- seven-sector area scenario S-DETAIL: **1249.97 m²**;
- seven-sector area scenario S-AGG: **1299.40 m²**;
- arithmetic residual outside reported sector areas: **1693.90–1743.33 m²**.

The technical relation also estimates a total swale influence area of approximately **228.8 m²**.

These quantities establish only **area plausibility**. They do not identify a scientifically admissible control polygon.

## 2. Comparator freeze objective

P1.3J shall produce one of two explicit terminal decisions before confirmatory outcome interpretation:

### J-INT — internal contemporaneous control frozen

A closed internal polygon is versioned and passes P1.3D C1–C5.

or

### J-EXT — internal control pathway failed / external reference required

The internal route is explicitly failed for a documented reason and a matched external reference polygon is frozen before confirmatory outcome evaluation.

A non-decision is not a freeze state.

## 3. Internal C-candidate geometry requirements

A C-candidate polygon must be reconstructed from vector/source evidence and must include:

- polygon ID and version;
- source geometry lineage;
- local metric area;
- usable area after buffers;
- minimum width / geometry constraints relevant to sampling;
- nearest irrigation infrastructure;
- relationship to swale/hydrological corridor;
- building/access/edge exclusions;
- known management history;
- intended no-treatment period;
- allowed protective maintenance;
- prohibited focal-treatment operations;
- contamination/spillover triggers.

No arithmetic residual or visually blank map area may be substituted for a polygon.

## 4. Exposure-overlay rules

The following layers must be overlaid before a polygon can pass C3:

### T1 direct exposure

- irrigation laterals and local irrigation infrastructure;
- swale body / direct earthwork;
- imported-soil placement;
- compost/amendment placement;
- mulch/cover-crop intervention;
- planting/bed construction;
- other focal regenerative treatment.

### T2 indirect influence

- modeled/estimated swale wetting and hydrological redistribution;
- runoff/convergence;
- irrigation wetting beyond source linework;
- root/canopy/shading where material to the endpoint;
- nutrient transport;
- access/traffic/maintenance spillover;
- edge effects.

Where T2 extent cannot be bounded from current evidence, uncertainty must be explicit and the control gate remains open.

## 5. Internal-control acceptance matrix

| Criterion | Freeze evidence | Current state |
|---|---|---|
| C1 sufficient usable area | closed polygon + buffered usable area + sampling capacity | PLAUSIBLE / OPEN |
| C2 baseline comparability | matched baseline table for primary endpoints and covariates | OPEN |
| C3 spillover isolation | T1/T2 overlay + buffer rationale | NOT PASSED |
| C4 operational durability | signed/recorded reservation window + management rule | OPEN |
| C5 agronomic/operational acceptability | steward/agronomic review | OPEN |

Internal control is frozen only if all five pass.

## 6. External-reference pathway activated in parallel

Because C3 remains unresolved, the external-reference pathway is now active as a **fallback workstream**, not as a selected comparator.

An external reference candidate must be screened for:

- same bioregional/climatic context;
- comparable soil/parent-material context where evidenced;
- comparable topographic position, slope and exposure;
- comparable prior disturbance/land-use state;
- similar starting vegetation/ground-cover condition;
- absence of the focal Genesis intervention;
- practical access for repeated sampling;
- stable management during the comparison window;
- consent/access and evidence provenance.

Adjacency alone does not establish comparability.

## 7. Baseline matching package

For every surviving internal or external candidate, collect the same pre-intervention package:

1. infiltration endpoint observations;
2. aggregate-stability endpoint observations;
3. SOC concentration and sampling-depth record;
4. bulk density if SOC stock will be used;
5. soil moisture at measurement;
6. ground-cover/vegetation state;
7. topographic/hydrologic position;
8. disturbance and management history;
9. photographic/field provenance;
10. coordinates with declared CRS/state.

Comparator choice must be made using prespecified matching variables and not by selecting the location that later gives the preferred outcome.

## 8. Control reservation record

If an internal candidate passes C1–C5, create:

`GENESIS_CONTROL_RESERVATION_<version>`

with:

- geometry hash;
- area;
- reservation start;
- primary comparison window;
- prohibited treatment list;
- permitted minimum maintenance;
- responsible steward;
- contamination/deviation rule;
- release conditions;
- reviewer acknowledgment.

The control remains part of the pilot site but outside the focal treatment for the declared comparison window.

## 9. Failure rule

The internal route must be marked **FAILED FOR CONFIRMATORY CONTROL** if any critical criterion cannot be satisfied before focal treatment contaminates the candidate polygon.

Failure is not hidden by relabelling the area as a control after treatment begins.

If this occurs, Prometheus shall use:

- repeated pre-intervention treatment baseline;
- matched external reference where feasible;
- bounded causal language;
- explicit quasi-experimental/observational classification where design limits require it.

## 10. Sampling freeze dependency

Exact sampling coordinates remain blocked until a comparator path reaches J-INT or J-EXT freeze.

Once frozen:

`Comparator polygon → strata → independent sampling units → sample-count/precision rationale → frozen coordinates → baseline collection`.

Subsamples inside one management unit must not be misrepresented as independent treatment-level replication.

## 11. Immediate evidence request / field action

The next field/CAD action is to identify every contiguous residual polygon that is intentionally capable of remaining outside:

- irrigation;
- imported soil;
- swale/earthwork;
- compost/amendment;
- mulch/cover-crop treatment;
- focal planting operations

for the primary comparison window.

If no such polygon exists, record that fact and move to J-EXT.

## 12. Claim boundary

This workpack governs comparator selection. It does not establish ecological efficacy, causal effect, field validation, certification, legal cadastral geometry, PRU/TRBK value or capital-facing admissibility.
