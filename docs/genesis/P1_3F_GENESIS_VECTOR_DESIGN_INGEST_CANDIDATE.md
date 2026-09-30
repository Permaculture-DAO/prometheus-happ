# PROMETHEUS P1.3F — Genesis Vector Design Ingest Candidate

Status: **CANDIDATE — VECTOR PROJECT DXF INGESTED; FEATURE ANCHORS RESOLVED; SECTOR POLYGONS + METRIC SCALE QA OPEN**  
Date: 2026-09-30

## 1. Source

User supplied the vector design file:

`progetto irriguo.dxf`\n\nSHA-256: `1c99461566ff015f61db8aa1f922c7d02e838a568a1356db1c2cd06c7b48222f`\n\nThis is materially different from the raster image previously embedded in the technical report: the irrigation project is now available as vector evidence.

## 2. Vector feature anchors confirmed

The DXF contains explicit vector/text anchors for the designed irrigation system and hydrological works.

### Hydraulic / swale anchors

| Feature | DXF handle | drawing X | drawing Y |
|---|---|---:|---:|
| INGRESSO SWALE | `7C2DCF` | 211.223153 | 321.347812 |
| USCITA SWALE | `7C4647` | 270.557987 | 232.538718 |
| SWALE label | `7C6E91` | 283.284551 | 207.061162 |
| INIZIO SWALE | `7C6E92` | 280.370227 | 272.500115 |
| POMPA PER LAGHETTO A VALLE | `7C4629` | 267.649685 | 234.078422 |

### Irrigation-sector valve anchors

| Sector / valve label | DXF handle | drawing X | drawing Y |
|---|---|---:|---:|
| aromatiche + frutteto-prato Est | `7C58FB` | 295.927163 | 307.762564 |
| siepe Est | `7C6EA1` | 295.928179 | 307.574735 |
| colture arboree | `7C6EA2` | 295.927262 | 307.956993 |
| siepe Ovest | `7C7791` | 292.072762 | 307.944440 |
| orti a lasagna | `7C77ED` | 291.950005 | 307.540280 |
| frutteto-prato Nord Ovest | `7C77EE` | 290.630416 | 307.812435 |
| frutteto-prato Sud Ovest | `7C77EF` | 290.758424 | 307.677685 |

The `PVC-DISTRIBUZIONE` layer contains vector network geometry and valve/supply symbols. The presence of a label or hydraulic line is evidence of designed geometry; it is not yet by itself a frozen sector polygon.

## 3. Scale/unit QA

The DXF header reports `$INSUNITS = 4` (AutoCAD code for millimetres), while a drawing annotation in the source explicitly states:

`Disegno in scala 1:1 (metri)`

Because the file also contains multiple blocks/copies and mixed architectural content, Prometheus does **not** silently resolve this metadata conflict.

Before using raw drawing coordinates as canonical metre geometry, P1.3F requires a scale-control check against independently known dimensions/areas from the survey/technical design.

Current state:

- vector topology: **AVAILABLE**;
- feature identity: **RESOLVED for named anchors**;
- metric scale interpretation: **QA OPEN**;
- CRS/georeferencing: **OPEN**.

## 4. Sector geometry gate

The relation defines seven management/irrigation sectors and their intended areas. The new vector file supplies the network and named valve anchors, but exact sector polygons must now be reconstructed from explicit vector boundaries rather than inferred from report raster pixels.

No polygon will be promoted merely by enclosing nearby irrigation lines.

Each sector polygon must satisfy:

1. closed, reproducible vector boundary;
2. traceable DXF handles/layers;
3. area reconciliation against the technical relation;
4. non-overlap/intentional-overlap classification;
5. hydraulic connection to the corresponding valve/network;
6. treatment-exposure classification (T1/T2/C).

## 5. S5 discrepancy remains active

The technical relation contains a critical discrepancy:

- detailed S5 calculation: approximately **356.57 / 356.6 m²**;
- later aggregate formula: **406 m²**.

The new DXF allows this discrepancy to be tested geometrically, but it is not resolved merely by file availability.

P1.3F shall resolve S5 only after the corresponding closed polygon is identified and its metric scale is validated.

## 6. Comparator implications

The design DXF confirms that the site contains a distributed irrigation network and explicitly named sector valves. Therefore “not directly planted” or “not visibly worked” cannot automatically be treated as untreated control.

Candidate control geometry must be tested against:

- irrigation exposure;
- swale/hydrological coupling;
- imported-soil/earthwork exposure;
- amendments/compost/mulch/cover-crop exposure;
- root/canopy influence;
- access/traffic and edge effects;
- baseline topographic/soil comparability.

The P1.3D C1–C5 control-feasibility gate remains controlling.

## 7. Next deterministic execution

Next step is **P1.3G — Vector Polygon Reconstruction & Metric Reconciliation**:

1. isolate the authoritative plan instance/copy;
2. validate coordinate scale with known control dimensions;
3. reconstruct seven closed sector polygons;
4. reconstruct swale centerline/banks and direct hydrological influence geometry;
5. extract irrigation network by sector;
6. reconcile sector polygon areas with report values;
7. classify T1 direct treatment, T2 indirect influence, and C comparator candidates;
8. generate sampling strata only after geometry QA passes.

## 8. Claim boundary

Vector availability improves spatial evidence quality. It does not establish ecological efficacy, causal treatment effect, field validation, cadastral legal geometry, certification, PRU/TRBK value or capital-facing admissibility.
