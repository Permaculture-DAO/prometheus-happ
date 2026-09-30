# PROMETHEUS P1.3C — Genesis Outcome Framework & Measurement Registry Candidate

Status: **CANDIDATE — OUTCOME DOMAINS FROZEN; METRICS/INSTRUMENTS/POWER OPEN**  
Date: 2026-09-30

## 1. Purpose

This registry defines the minimum outcome architecture required before the Sicily Genesis Pilot can move from spatial/design preregistration to a field-measurement freeze.

It does not assert that any outcome has improved.

## 2. Design constraints inherited from P1.3/P1.3B

- ExperimentalSystemBoundary: surveyed DXF outline A8D, 3360.875 m².
- Comparator/reference geometry: not yet frozen.
- Sampling must account for terrain/spatial heterogeneity.
- Measurements require stable IDs, declared units/methods, timestamps, provenance and disturbance flags.
- Post-hoc promotion of exploratory metrics to preregistered primary outcomes is prohibited.

## 3. Outcome hierarchy

### Tier 1 — Primary outcome domains

The pilot must select a small set of field-measurable primary metrics from these domains before baseline:

1. **Soil state**
   - physical/hydrologic condition;
   - carbon/organic-matter related state where laboratory method is specified;
   - other soil variables only with defined method and units.

2. **Water / infiltration function**
   - a repeatable field measure of infiltration or related hydrologic response;
   - method, apparatus, antecedent conditions and replicate rule must be frozen.

3. **Vegetation / ground-cover structure**
   - a repeatable quantitative measure of vegetation/ground cover or structural state;
   - image-derived measures require a frozen acquisition and analysis protocol.

Primary status is assigned only after metric, unit, method, precision rationale and sampling plan are frozen.

### Tier 2 — Secondary outcome domains

Candidate secondary domains include:

- soil moisture;
- temperature/microclimate;
- vegetation diversity/richness indices;
- biomass/productivity proxies;
- erosion/soil-surface indicators;
- water-retention proxies;
- management/input intensity.

These are not automatically primary and cannot be used to rescue a failed primary hypothesis.

### Tier 3 — Context/covariates

Record where relevant:

- rainfall/weather;
- topographic position;
- slope/aspect;
- disturbance;
- management operations;
- access/traffic;
- prior land-use evidence;
- sensor status/calibration.

Context variables support interpretation; they are not evidence of efficacy by themselves.

## 4. Metric admission schema

Every metric proposed for baseline MUST have:

- `metric_uid`;
- domain and tier;
- human-readable definition;
- unit;
- method/SOP identifier;
- instrument or laboratory method;
- calibration/QA rule;
- spatial sampling unit;
- replicate rule;
- temporal window;
- detection/precision limits where relevant;
- missing/invalid observation rule;
- evidence/provenance type;
- treatment/comparator applicability;
- prespecified estimand;
- uncertainty-reporting method;
- falsification/non-support criterion;
- reviewer scope.

If any required field is unresolved, the metric remains `candidate`.

## 5. Primary endpoint freeze rule

A metric may become a preregistered primary endpoint only if all of the following are true:

1. the measurement can be repeated under a written SOP;
2. units and instrument/method are fixed;
3. sampling geometry and replicate rule are fixed;
4. baseline is collected before the focal intervention;
5. comparator/reference applicability is explicit;
6. estimand and uncertainty method are prespecified;
7. missing-data and disturbance handling are prespecified;
8. no baseline result has been used to cherry-pick the metric's primary status, except where the protocol explicitly declares a pilot/feasibility phase and preserves that distinction.

## 6. Observation record

Minimum field observation payload:

```text
observation_id
metric_uid
system_boundary_id
sampling_unit_id
treatment_status
timestamp
coordinates + CRS
value + unit
method_id
instrument_id / laboratory reference
operator/source
qa_status
disturbance_flags
missingness_status
evidence_reference
```

Raw observations remain distinct from derived statistics.

## 7. Baseline completeness gate

Baseline is complete only when, for every frozen primary endpoint:

- all required strata/units have scheduled observations;
- expected observations are accounted for as valid or explicitly missing/invalid;
- QA/calibration evidence is attached where required;
- disturbances are recorded;
- treatment has not contaminated the defined pre-intervention window;
- baseline EvidencePackage can be reconstructed from source observations.

## 8. Analysis and multiplicity discipline

The protocol must distinguish:

- primary confirmatory endpoints;
- secondary endpoints;
- exploratory endpoints.

If multiple primary endpoints are used, the final statistical analysis plan must state how multiplicity is handled or explicitly constrain the interpretation. Effect sizes and uncertainty intervals are required; a binary significance label alone is insufficient.

Spatial/repeated observations must not be treated as independent replicates when the design does not support that assumption.

## 9. Sensor and laboratory evidence boundary

A sensor reading or laboratory result is an observation with provenance, not ecological truth.

Where sensors are used, retain:

- device/model identifier;
- calibration/verification record;
- deployment location;
- clock/time basis;
- firmware/configuration where material;
- raw-data preservation rule;
- transformation/aggregation code version.

Where laboratory analyses are used, retain sample chain-of-custody and method/reference information when available.

## 10. Evidence Spine mapping

`ExperimentalSystemBoundary → SamplingUnit → Observation → EvidencePackage → ReviewAttestation → AdmissibilityDecision`

Derived metrics must remain traceable to the underlying Observation records and transformation version.

## 11. Immediate field-design decisions still required

Before P1.3C can be frozen:

1. select the actual primary metric(s), not merely domains;
2. define SOPs/instruments/laboratory methods;
3. decide internal control feasibility and comparator geometry;
4. acquire/verify canonical terrain data;
5. establish baseline and intervention dates;
6. determine sampling density/replication using a precision/power or explicit pilot rationale;
7. freeze sampling coordinates;
8. assign independent review scope.

## 12. Claim boundary

This registry is measurement governance. It does not establish field validation, ecological efficacy, scientific validation, certification, legal/market admission, or PRU/TRBK value.
