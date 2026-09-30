# PROMETHEUS P1.3U — Genesis Field Data Schema, QA & Chain-of-Custody Workpack

Status: **PREPARED — FIELD EXECUTION PENDING**  
Date: 2026-09-30

## 1. Purpose

Define the minimum field-data structure, quality controls and sample chain-of-custody required for Genesis and EXT-001 so every future observation can be reconstructed and audited.

## 2. Core field observation schema

Each observation record shall include:

- `observation_id`
- `site_id`
- `comparison_role` = GENESIS | EXT-001
- `sampling_point_id`
- `stratum_id`
- `metric_uid`
- `timestamp_local`
- `latitude`
- `longitude`
- `crs`
- `value`
- `unit`
- `method_id`
- `instrument_id`
- `operator_id`
- `qa_status`
- `disturbance_flag`
- `missingness_status`
- `photo_evidence_ref`
- `source_file_hash`
- `notes`

## 3. Soil-sample schema

For each physical sample:

- `sample_id`
- `site_id`
- `sampling_point_id`
- `depth_top_cm`
- `depth_bottom_cm`
- `collection_timestamp`
- `container_id`
- `collector`
- `field_condition`
- `storage_condition`
- `transfer_timestamp`
- `laboratory_id`
- `laboratory_method_id`
- `lab_report_ref`
- `chain_of_custody_status`
- `result_status`

## 4. Field QA states

Allowed QA states:

- `VALID`
- `VALID_WITH_NOTE`
- `INVALID_METHOD_DEVIATION`
- `INVALID_INSTRUMENT`
- `INVALID_DISTURBANCE`
- `MISSING`
- `PENDING_REVIEW`

Invalid values remain in the audit record; they are not deleted.

## 5. Missing-data discipline

Every expected observation must be represented as either:

- valid result;
- invalid result with reason;
- missing with reason.

No silent deletion or untracked imputation is allowed.

Any imputation used in analysis must be prespecified and accompanied by sensitivity analysis.

## 6. Disturbance ledger

Record separately:

- rainfall event;
- irrigation;
- tillage;
- grazing/mowing;
- fertilizer/amendment;
- pesticide/herbicide;
- earthwork;
- imported soil;
- traffic;
- fire;
- flooding/runoff;
- equipment failure;
- unauthorized treatment;
- other material disturbance.

Each disturbance record includes timestamp, affected geometry/point(s), source and expected endpoint relevance.

## 7. Photo evidence

At minimum:

- one plot-level overview;
- one ground-surface/nadir view where useful;
- one sampling-point photo before destructive sampling;
- one equipment/method setup photo for the first session of each SOP version.

Photos must preserve timestamp and point linkage where possible.

## 8. Chain-of-custody rule

A laboratory result cannot enter a reviewable EvidencePackage unless its physical sample can be traced from:

`field sample ID → collector → container → transfer → laboratory → method → report`.

Broken custody is not silently repaired; it becomes a QA limitation.

## 9. Raw vs derived data

Raw observations are immutable evidence inputs.

Derived statistics must preserve:

- source observation IDs;
- transformation code/version;
- method version;
- timestamp;
- analyst/reviewer;
- output hash where applicable.

## 10. Data package structure

Recommended field package:

`/genesis/<date>/`
- `field_observations.csv`
- `soil_samples.csv`
- `disturbances.csv`
- `photos/`
- `lab/`
- `methods/`
- `manifest.json`
- `SHA256SUMS.txt`

Equivalent structure for EXT-001.

## 11. Evidence Spine mapping

`SamplingUnit → Observation → EvidencePackage → ReviewAttestation → AdmissibilityDecision`

Field data collection does not itself create admissibility or PRU value.

## 12. Promotion gate

P1.3U is operationally ready when:

- final SOP IDs exist;
- sampling coordinates are frozen;
- field operators are assigned;
- sample labels/containers are prepared;
- laboratory path is confirmed;
- data templates are versioned.

## 13. Claim boundary

Data integrity improves auditability. It does not establish physical truth, causal effect, certification or financial value.
