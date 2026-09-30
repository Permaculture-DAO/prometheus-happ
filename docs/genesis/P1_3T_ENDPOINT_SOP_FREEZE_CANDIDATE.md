# PROMETHEUS P1.3T — Genesis / EXT-001 Endpoint SOP Freeze Candidate

Status: **CANDIDATE — METHODS SELECTED IN PRINCIPLE; EQUIPMENT/LAB IDs + FIELD TRIAL REQUIRED BEFORE FINAL FREEZE**  
Date: 2026-09-30

## 1. Purpose

Convert P1.3D primary endpoint candidates into one consistent measurement pathway to be applied identically to Genesis and EXT-001.

No method may differ by treatment/reference status.

## 2. PE-01 — Infiltration function

### Preferred method

Use a repeatable ring-infiltrometer protocol with a fixed apparatus and water-head/volume rule at both sites.

Preferred implementation:

- double-ring infiltrometer where field logistics allow;
- same ring dimensions for Genesis and EXT-001;
- fixed pre-wetting/initial-condition rule;
- record cumulative infiltration versus elapsed time;
- derive the prespecified infiltration response metric from the same time series;
- record antecedent soil moisture and recent rainfall/irrigation.

### Primary metric candidate

`PE01_INF_RATE` — infiltration rate in mm/h calculated under the frozen field protocol.

Secondary raw/derived variables:

- cumulative infiltration;
- time to infiltrate prespecified depth/volume;
- initial and late-interval rates;
- antecedent volumetric/gravimetric moisture;
- validity/disturbance flag.

### Freeze blockers

- exact ring dimensions;
- exact water-head or fixed-volume rule;
- pre-wetting rule;
- termination criterion;
- field trial repeatability;
- equipment ID.

## 3. PE-02 — Aggregate stability

### Preferred method

Use one standardized wet-sieving / water-stable aggregate procedure through the same laboratory or the same controlled field/lab apparatus for Genesis and EXT-001.

### Primary metric candidate

`PE02_WSA` — water-stable aggregate fraction/percentage under the frozen method.

### Sampling candidate

- depth: **0–10 cm**;
- fixed sample mass/preparation;
- fixed moisture-handling rule;
- same sieve/method;
- same analytical pipeline;
- destructive sample offset from repeated infiltration location.

### Freeze blockers

- laboratory/method ID;
- sample mass;
- sieve sequence;
- compositing/replicate rule;
- precision/QA acceptance.

## 4. PE-03 — Soil organic carbon

### Primary state metric

`PE03_SOC_CONC` — SOC concentration measured by one declared laboratory method.

Candidate depth increments:

- **0–10 cm**
- **10–30 cm**

The same depths must be applied at Genesis and EXT-001.

### Stock metric boundary

SOC stock may be calculated only if bulk density and coarse-fragment handling are measured appropriately.

Because the Genesis design contemplates substantial imported fertile soil in treatment sectors, fixed-depth SOC stock can be confounded by added soil mass.

Therefore:

- SOC concentration is the primary soil-carbon state endpoint candidate;
- SOC stock is secondary until bulk density/mass accounting is complete;
- any later sequestration claim must distinguish carbon imported with soil/compost from carbon accumulated in situ;
- imported organic carbon cannot be silently counted as ecosystem sequestration.

Where feasible, an equivalent-soil-mass sensitivity analysis should be considered for stock interpretation.

## 5. Companion measurements

At every primary sampling unit record:

- soil moisture;
- ground-cover/vegetation state;
- recent disturbance;
- treatment/reference status;
- topographic/hydrologic stratum;
- photo/evidence reference;
- rainfall/irrigation context;
- operator;
- timestamp.

## 6. Sample identity

Every physical soil sample receives:

- site ID;
- candidate/comparator ID;
- point ID;
- depth;
- date/time;
- operator;
- sample/container ID;
- chain-of-custody state;
- lab/method ID;
- evidence reference.

## 7. Method lock rule

The final SOP version must be frozen before confirmatory baseline.

After baseline starts, method changes require:

- protocol-deviation record;
- reason;
- affected endpoints/observations;
- comparability assessment;
- remeasurement or sensitivity plan where needed.

## 8. Field trial rule

One short feasibility session may be used to test equipment, timing and sample handling.

Those measurements must be labeled `FEASIBILITY_ONLY` and cannot be silently promoted into the confirmatory baseline unless the protocol had already been frozen before collection.

## 9. Claim boundary

A laboratory value or field instrument output is an observation with provenance. It is not, by itself, proof of regeneration, causality, certification, sequestration or PRU value.
