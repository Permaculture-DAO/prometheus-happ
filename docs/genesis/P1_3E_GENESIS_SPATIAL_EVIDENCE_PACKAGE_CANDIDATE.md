# PROMETHEUS P1.3E — Genesis Spatial Evidence Package Candidate

Status: **CANDIDATE — SURVEYED SYSTEM BOUNDARY HASHED; DESIGN PLAN RASTER-ONLY; SECTOR/CONTROL GEOMETRY NOT FROZEN**  
Date: 2026-09-30

## 1. Reproducible source state

The currently supplied spatial/design evidence has been independently hashed:

| Evidence object | SHA-256 | State |
|---|---|---|
| `RILIEVO TERRENO.dxf` | `848192a1c3e43ef060df948929a49bd78eee8b8ae83860e2fda01e90be70cf70` | survey/vector evidence |
| `Relazione tecnica.docx` | `c9b110ad8b0ed3fcaceffeec05ee71b1c67907b84ae605fb49d372b612daceb5` | technical-design evidence |
| `Relazione agronomica di invarianza idraulica e idrologica.docx` | `6a29755e9fa8e13dc1ecb7e4737c7de2852852e6251bf18d2aaf181f7c6a8889` | agronomic/hydrologic evidence |
| embedded project plan `image8.jpeg` | `aed6528ee2b296c6bab63b81f5c224c7ea4a0ccbd40539e6d50a493951b8b2e1` | raster design evidence only |

The embedded project plan is 1237 × 531 px. Pixel geometry is not treated as metric or georeferenced geometry.

## 2. Surveyed ExperimentalSystemBoundary

DXF handle `A8D` remains the frozen surveyed physical ExperimentalSystemBoundary.

- drawing units: metres (`$INSUNITS = 6`);
- 13 distinct vertices plus repeated closing record;
- independently calculated area: **3360.875 m²**;
- independently calculated perimeter: **257.426 m**;
- coordinate state: **drawing coordinates / CRS unresolved**.

No WGS84/EPSG transformation is asserted.

## 3. Vector-design availability gate

The currently available survey DXF does not contain a verified vector representation of the seven irrigation-sector polygons, swale design, irrigation network and treatment footprints shown in the technical project's embedded raster plan.

The survey DXF and the raster project plan are therefore treated as separate evidence objects.

P1.3E MUST NOT infer exact sector polygons or sampling coordinates from raster pixels.

## 4. Terrain-point caution

The survey DXF contains labelled survey points 100–123 with elevation labels approximately 82–95 m. In the inspected drawing state, their insert coordinates are not spatially contained by A8D.

Therefore these elevation records remain **EVIDENCE_ONLY / UNBOUND_TO_A8D** until the relevant drawing copy, transform or coordinate relationship is verified.

They MUST NOT yet be interpolated into canonical Genesis terrain strata.

## 5. Sector-area QA

The technical relation supports detailed sector areas of:

- S1: 150 m²;
- S2: 235 m²;
- S3: 120 m²;
- S4: 180 m²;
- S5: 356.57 m² in the detailed calculation;
- S6: 172 m²;
- S7: 36.4 m².

The detailed values sum to **1249.97 m²**.

A later aggregate calculation uses **406 m²** for S5 and approximately **1300 m²** total. The S5 `356.57/356.6 vs 406 m²` discrepancy remains a **critical spatial freeze blocker**.

## 6. Comparator consequence

The seven irrigation sectors are management strata, not independent experimental replicates.

An internal untreated control is not established by the current raster plan. Project-wide hydrological and regenerative interventions create plausible direct and indirect spillover.

Comparator status therefore remains subject to the P1.3D C1–C5 feasibility gate and requires explicit vector geometry.

## 7. Promotion gate

P1.3E may move to spatial-design freeze only after the original vector CAD/DXF/DWG of the project design is supplied or an explicitly derived, registered digitization is produced with positional uncertainty.

The next spatial package must extract and version:

1. seven irrigation-sector polygons;
2. swale centerline/banks and hydrological influence geometry;
3. irrigation network;
4. buildings/impermeable/access exclusion polygons;
5. direct-treatment and indirect-influence polygons;
6. candidate comparator/control polygon(s);
7. sampling-point coordinates after comparator and endpoint freeze.

If raster digitization becomes necessary, it must be registered to independently verified control points and labelled **DERIVED**, never source-equivalent.

## 8. Claim boundary

This package establishes provenance and spatial QA only.

It does not establish ecological efficacy, Sicily field validation, certification, legal cadastral geometry, PRU/TRBK value, market admission or capital-facing admissibility.

Cryptographic integrity and geometric reproducibility do not establish physical/ecological truth.
