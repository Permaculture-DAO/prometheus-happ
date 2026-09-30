# PROMETHEUS P1.3S — Genesis / EXT-001 Sampling Frame & Replication Boundary

Status: **CANDIDATE — RELATIVE SAMPLING ARCHITECTURE FROZEN IN PRINCIPLE; CONFIRMATORY COORDINATES OPEN**  
Date: 2026-09-30

## 1. Objective

Define a spatial sampling architecture that is operationally repeatable while preserving the distinction between:

- site/field-level experimental units;
- spatial sampling units;
- technical replicates;
- repeated temporal observations.

## 2. External comparator frame

EXT-001 provisional frame:

- gross plot: 30 × 30 m = 900 m²;
- analysis core: 20 × 20 m = 400 m²;
- nominal internal buffer: 5 m.

Candidate relative grid inside the 20 × 20 m core:

| Point | Easting in core (m) | Northing in core (m) |
|---|---:|---:|
| P01 | 2.5 | 2.5 |
| P02 | 7.5 | 2.5 |
| P03 | 12.5 | 2.5 |
| P04 | 17.5 | 2.5 |
| P05 | 2.5 | 7.5 |
| P06 | 7.5 | 7.5 |
| P07 | 12.5 | 7.5 |
| P08 | 17.5 | 7.5 |
| P09 | 2.5 | 12.5 |
| P10 | 7.5 | 12.5 |
| P11 | 12.5 | 12.5 |
| P12 | 17.5 | 12.5 |
| P13 | 2.5 | 17.5 |
| P14 | 7.5 | 17.5 |
| P15 | 12.5 | 17.5 |
| P16 | 17.5 | 17.5 |

This 4 × 4 lattice is a **candidate feasibility grid**.

Coordinates must not be promoted to confirmatory field coordinates until the 20 × 20 m core is verified inside usable Particella 849 geometry and local exclusions are mapped.

## 3. Genesis treatment-side sampling architecture

The Genesis site must not be represented by one convenient corner.

The treatment-side sampling frame shall be stratified across the ExperimentalSystemBoundary using the final spatial strata available before baseline.

Minimum candidate architecture:

- four treatment strata or spatial blocks where defensible;
- four spatial sampling units per stratum;
- target total: 16 treatment-side spatial points.

If fewer than four defensible strata exist, points must still be distributed across the main treatment heterogeneity and the limitation disclosed.

Exact Genesis coordinates remain OPEN because treatment strata, sector polygons and terrain matching are not yet fully frozen.

## 4. Endpoint co-location

Preferred field architecture:

- PE-01 infiltration at each primary spatial point;
- PE-02 aggregate-stability sample co-located or within a fixed small offset;
- PE-03 SOC sample co-located or combined under a prespecified compositing rule;
- soil moisture measured at or immediately adjacent to the infiltration point;
- ground cover recorded at the same sampling unit.

Destructive sampling offsets must be fixed in the SOP to avoid disturbing later repeated measurements.

## 5. Replication boundary

### Site-level units

- Genesis: one treated site;
- EXT-001: one external reference field/plot.

Therefore:

**field-level treatment replication = 1 treated site + 1 external reference site**

The 16 spatial points in each land unit are **spatial subsamples**, not 16 independent treatment replicates.

### Consequence

The design can support:

- site-specific change estimation;
- spatially resolved treatment/reference contrasts;
- repeated-measures evidence;
- pilot effect-size and variance estimation;
- future replication planning.

It cannot by itself support unrestricted population-level causal generalization about all regenerative/syntropic systems.

## 6. Future strengthening

If additional neighboring reference plots become available, retain them as independent external reference units rather than merely adding more subsamples to EXT-001.

Future independent treated Genesis-like sites are required for broader treatment-level replication.

## 7. Sampling-count freeze rule

The 16 + 16 spatial-point architecture is a **candidate minimum practical design**, not a final power calculation.

Final count must consider:

- baseline variance;
- spatial autocorrelation;
- minimum detectable/meaningful change;
- endpoint measurement error;
- attrition/missingness;
- field feasibility.

If precision analysis requires more points, increase density before confirmatory baseline rather than after inspecting outcomes.

## 8. Coordinate freeze gate

Confirmatory coordinates may be frozen only after:

1. EXT-001 geometry is field-verified;
2. Genesis strata are frozen;
3. exclusion zones are mapped;
4. endpoint SOPs are frozen;
5. precision/variance rationale is documented.

## 9. Claim boundary

Sampling density does not create treatment-level replication. Spatial subsampling must never be presented as multiple independent sites.
