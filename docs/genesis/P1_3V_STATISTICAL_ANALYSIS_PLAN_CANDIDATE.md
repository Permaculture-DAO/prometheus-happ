# PROMETHEUS P1.3V — Genesis Statistical Analysis Plan Candidate

Status: **CANDIDATE — ANALYSIS LOGIC FROZEN IN PRINCIPLE; BASELINE VARIANCE + FINAL COMPARATOR QUALIFICATION OPEN**  
Date: 2026-09-30

## 1. Objective

Prespecify how Genesis and EXT-001 observations will be summarized and compared without overstating treatment replication.

## 2. Design class

Current design:

- one treated Genesis site;
- one matched external reference candidate, EXT-001;
- repeated spatial sampling within each land unit;
- repeated temporal observations.

This is a **paired-site / quasi-experimental repeated-measures pilot design**, not a replicated randomized field experiment.

## 3. Primary estimand form

For each endpoint where baseline + follow-up are available, candidate site-specific estimand:

`Δ_Genesis − Δ_EXT001`

where:

- `Δ_Genesis` = post minus baseline change in Genesis;
- `Δ_EXT001` = post minus baseline change in the matched external reference.

This is a site-specific difference-in-change estimand.

## 4. Primary endpoints

### PE-01
Infiltration response.

Report:

- baseline distribution;
- follow-up distribution;
- site-level mean/median change as prespecified;
- Genesis-minus-reference change contrast;
- uncertainty interval;
- spatial/temporal sensitivity.

### PE-02
Water-stable aggregate metric.

Same comparison structure.

### PE-03
SOC concentration at fixed depth increments.

Report treatment/reference change by depth.

SOC stock remains secondary until bulk density and imported-carbon accounting are adequate.

## 5. Statistical unit boundary

The site/field is the highest-level land unit.

Spatial sampling points are nested subsamples.

Repeated visits are repeated observations on the same spatial support.

No analysis may treat 16 points inside Genesis as 16 independent treated sites or 16 points inside EXT-001 as 16 independent control sites.

## 6. Model candidate

Where data density supports it, use a repeated/spatial hierarchical model with:

- fixed terms for site role, time and site-role × time;
- point/stratum structure where appropriate;
- repeated-measure correlation;
- spatial correlation considered when supported by coordinates/data.

If sample size is insufficient for a stable hierarchical model, use transparent descriptive/effect-size analysis rather than forcing an unstable model.

## 7. Uncertainty

Report:

- effect estimate;
- interval estimate;
- raw data distribution;
- point/stratum count;
- missingness;
- sensitivity to outliers/deviations;
- model assumptions.

A binary p-value is not sufficient.

## 8. Multiplicity

Primary endpoint interpretation must be prespecified.

If PE-01, PE-02 and PE-03 all remain primary, the final analysis must either:

- apply a declared multiplicity-control approach; or
- define a hierarchical/ordered primary interpretation; or
- explicitly classify the pilot as estimation-focused rather than confirmatory hypothesis testing.

Current preferred pilot posture: **estimation-focused**, emphasizing effect sizes, uncertainty and reproducibility rather than a binary pass/fail significance threshold.

## 9. Baseline imbalance

Material baseline differences between Genesis and EXT-001 must be:

- reported;
- incorporated as covariates only if prespecified and statistically defensible;
- tested in sensitivity analysis;
- never hidden by re-selecting the comparator after outcomes are known.

## 10. Missing data / disturbances

Primary analysis will not silently impute missing observations.

Sensitivity analysis is required where missingness, disturbance or management deviations could materially affect the result.

## 11. Imported soil / carbon accounting

Because Genesis contemplates imported fertile soil:

- imported soil quantity/date/source must enter the intervention ledger;
- carbon contained in imported soil/compost must not be interpreted as in-situ sequestration;
- SOC concentration change may still be reported as a soil-state change;
- stock/sequestration language requires separate mass-balance/admissibility review.

## 12. H1/H2/H5 claim boundaries

### H1 / H2
The pilot may support site-specific evidence of soil/water change relative to the matched reference if protocol, comparator and evidence gates are satisfied.

It does not by itself establish universal treatment efficacy.

### H5
A single treated site plus one external reference does not provide independent treatment-level replication sufficient for broad confirmatory cooperative-syntropic-surplus claims.

H5 remains research/pilot-bound unless the required independent replication architecture is later achieved.

## 13. Analysis freeze rule

Final SAP freeze requires:

- EXT-001 retained as comparator or replacement comparator frozen;
- final primary endpoint definitions;
- SOP versions;
- baseline variance/precision rationale;
- sampling coordinates;
- observation calendar;
- reviewer assignment.

## 14. Claim boundary

Statistical analysis structures evidence. It does not create ecological truth, certification, admissibility or financial value.
