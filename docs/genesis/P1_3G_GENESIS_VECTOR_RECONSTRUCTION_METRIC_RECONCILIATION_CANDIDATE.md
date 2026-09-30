# PROMETHEUS P1.3G — Genesis Vector Reconstruction & Metric Reconciliation Candidate

Status: **CANDIDATE — LOCAL METRIC SCALE QA PASSED; HYDRAULIC SUBARRAYS RECONSTRUCTED; SECTOR POLYGONS/CONTROL GEOMETRY OPEN**  
Date: 2026-09-30

## 1. Source lineage

Primary vector source:

- file: `progetto irriguo.dxf`
- SHA-256: `1c99461566ff015f61db8aa1f922c7d02e838a568a1356db1c2cd06c7b48222f`

Inherited surveyed boundary:

- ExperimentalSystemBoundary: survey DXF handle `A8D`
- area: **3360.875 m²**
- perimeter: **257.426 m**
- survey CRS: unresolved / drawing-coordinate evidence only.

## 2. Local metric-scale QA

The irrigation-project DXF header reports `$INSUNITS = 4` (millimetres), but the drawing itself explicitly contains the annotation:

`Disegno in scala 1:1 (metri)`

A vector frame on layer `R2V` provides an independent numerical scale-control object:

- left vertical handle `7C4619`: **92.000000 m drawing length**
- right vertical handle `7C461C`: **92.000000 m drawing length**
- lower horizontal handle `7C461D`: **37.0198775 m drawing length**
- upper horizontal handle `7C461A`: **37.0198775 m drawing length**

Frame dimensions: **37.0198775 × 92.000000 drawing units**.

If one drawing unit is interpreted as one metre:

- frame area = **3405.8287 m²**
- frame perimeter = **258.0398 m**

Relative to the independently reconstructed surveyed A8D geometry:

- area difference = **+44.9537 m² (+1.338%)**
- perimeter difference = **+0.6138 m (+0.238%)**

This convergence, combined with the explicit 1:1-metres drawing annotation and site-scale geometry, is sufficient to treat **one local design drawing unit as one metre for internal metric measurements**.

### Boundary

This resolves **local design scale**, not CRS/georeferencing and not legal/survey boundary identity.

The `R2V` frame is used as a scale-control object only. It is not promoted as the ExperimentalSystemBoundary and does not replace A8D.

The conflicting `$INSUNITS=4` header is retained as a source-metadata discrepancy.

## 3. Source-vector hydraulic reconstruction

The `PVC-DISTRIBUZIONE` layer contains reproducible vector irrigation geometry. Three dense lateral arrays can be reconstructed directly from source LINE entities without raster digitisation.

### HZ-NW-01 — north-west lateral array

- source layer: `PVC-DISTRIBUZIONE`
- source LINE entities: **24**
- distinct lateral Y levels: **23**
- lateral spacing: **0.600 m** across 22 consecutive intervals
- X extent: **260.567041 → 268.767041**
- Y extent: **284.424750 → 297.624750**
- source-envelope dimensions: approximately **8.200 × 13.200 m**

This is a hydraulic subarray, not yet asserted to be the complete S2 management polygon.

### HZ-SW-01 — south-west lateral array

- source LINE entities: **19**
- distinct lateral Y levels: **19**
- lateral spacing: **0.600 m** across 18 consecutive intervals
- X extent: **263.992702 → 273.592723**
- Y extent: **268.976558 → 279.776558**
- source-envelope dimensions: approximately **9.600 × 10.800 m**

This is a hydraulic subarray, not yet asserted to be the complete S3 management polygon.

### HZ-ELOW-01 — eastern/lower lateral array

- source LINE entities: **30**
- distinct lateral Y levels: **15**
- paired/duplicated lateral geometry at several levels
- lateral spacing: **0.600 m** across 14 consecutive intervals
- X extent: **277.133297 → 295.557315**
- Y extent: **261.524750 → 269.924750**
- maximum source-envelope dimensions: approximately **18.424 × 8.400 m**

This source geometry is a hydraulic subarray. Its final mapping to a specific management sector must follow branch/valve topology and the technical plan; proximity alone is insufficient.

## 4. Main hydraulic trunks and distribution evidence

Reconstructed source features include:

- west/upper distribution line: handle `7CB818`, vertical length **88.0 m**;
- east distribution line: handle `7CB961`, vertical length approximately **64.0 m**;
- eastern continuation: handle `7C77CE`, length approximately **61.006 m**;
- top distribution lines including handles `7C77C8`, `7C7804`, `7C77C0`, `7C77C3`;
- named valve anchors for all seven irrigation sectors;
- vector swale labels/anchors and downstream pond-pump anchor.

The network is therefore source-vector evidence, not a raster inference.

## 5. Management-sector polygon state

The technical report defines seven irrigation/management sectors, but the source DXF does not encode them as seven simple closed `LWPOLYLINE` polygons.

Accordingly:

- seven sector identities: **RESOLVED**
- valve anchors: **RESOLVED**
- irrigation linework: **RESOLVED**
- local metric scale: **RESOLVED FOR DESIGN MEASUREMENT**
- exact seven management-sector polygons: **OPEN / DERIVED RECONSTRUCTION REQUIRED**
- CRS/georeferencing to survey coordinates: **OPEN**
- treatment/control polygons: **OPEN**

No polygon is manufactured merely by buffering irrigation lines or enclosing nearby vegetation.

## 6. S5 area reconciliation

The source technical relation still contains:

- S5 detailed calculation: **356.57 / 356.6 m²**
- later aggregate calculation: **406 m²**

The detailed seven-sector values sum to **1249.97 m²**; the later aggregate expression using 406 m² produces approximately **1299.4 m²**, reported as about 1300 m².

P1.3G does **not** silently choose one value.

S5 remains a critical reconciliation item until the corresponding management footprint is traced from source vector topology and its area calculated under the verified local metric scale.

## 7. Experimental-control consequence

The source vector network confirms distributed hydraulic intervention across the site.

Therefore a candidate internal control must be screened against both:

- **T1 direct exposure** — irrigation, swale/earthwork, imported soil, amendment, planting, mulch/cover crop or other focal treatment;
- **T2 indirect exposure** — hydrological redistribution, runoff, root/canopy influence, nutrient movement, access/traffic, edge and management spillover.

The P1.3D C1–C5 comparator gate remains controlling.

A non-irrigated-looking residual area is not automatically an untreated control.

## 8. Next deterministic step — P1.3H

Proceed to **P1.3H — Treatment Exposure Map & Comparator Candidate**.

Required outputs:

1. trace source-connected hydraulic branches from the seven valve groups;
2. derive source-linked hydraulic exposure envelopes with explicit DERIVED status;
3. reconstruct swale centerline/banks and a tiered hydrological-influence classification;
4. mark buildings, impermeable surfaces, access and infrastructure exclusions;
5. classify residual land as T1 / T2 / C-candidate / excluded;
6. test each C-candidate against P1.3D C1–C5;
7. only then generate candidate sampling strata and point locations.

## 9. Claim boundary

P1.3G establishes vector reconstruction and local metric QA.

It does not establish ecological efficacy, causal effect, field validation, certification, legal cadastral geometry, PRU/TRBK value or capital-facing admissibility.
