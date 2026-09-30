# PROMETHEUS P1.3L-EXT — Genesis Matched External Reference Selection Protocol

Status: **ROUTE SELECTED — INTERNAL CONTROL CLOSED; MATCHED NEIGHBOR-LAND REFERENCE REQUIRED; GEOMETRY OPEN**  
Date: 2026-09-30

## 1. Comparator-route decision

The site steward has decided that no portion of the Genesis ExperimentalSystemBoundary will be intentionally withheld from the focal regenerative treatment for the primary scientific comparison window.

Accordingly:

- **J-INT internal contemporaneous control: CLOSED / NOT SELECTED**
- **J-EXT matched external reference: SELECTED**
- repeated pre-intervention Genesis baseline: **REQUIRED**
- exact external comparator polygon(s): **OPEN**

This is a prospective design decision made before confirmatory outcome interpretation.

## 2. Comparator class

The primary contemporaneous comparator shall be one or more neighboring or nearby land parcels/plots that remain outside the focal Genesis regenerative intervention and are matched as closely as practicable on pre-specified baseline characteristics.

A neighboring field is not automatically a valid comparator merely because it is adjacent.

## 3. Mandatory external-reference eligibility screen

Each candidate neighboring plot must be screened before acceptance against the following dimensions:

1. **Bioregional/climatic comparability**
   - same local climatic exposure as far as practicable;
   - similar rainfall, wind and coastal exposure context.

2. **Soil / parent-material comparability**
   - comparable soil texture/profile where evidenced;
   - comparable parent material/geological context where evidenced;
   - no known major fill, excavation or imported-soil history that would invalidate matching.

3. **Topographic comparability**
   - similar elevation band;
   - similar slope magnitude;
   - similar aspect/exposure where material;
   - similar upslope/downslope hydrological position.

4. **Land-use and disturbance history**
   - prior cultivation/abandonment/disturbance history documented as far as practicable;
   - no recent major treatment that would materially bias the selected primary endpoints.

5. **Vegetation / ground-cover baseline**
   - comparable starting cover and structural state;
   - material differences must be measured and reported.

6. **Hydrological independence from Genesis**
   - the comparator must not receive runoff, irrigation, swale discharge, amendment transport or other material treatment spillover from Genesis.

7. **Management stability**
   - owner/operator must be willing to avoid major management changes during the declared comparison window or record them as protocol deviations.

8. **Access and repeatability**
   - repeated access for the same measurement schedule must be feasible;
   - sampling must be authorized.

9. **Measurement-method identity**
   - the same SOPs, instruments/laboratory methods, depths and timing rules must be used in Genesis and comparator observations.

10. **Provenance**
   - geometry, landowner permission/access status, observation source and management-history evidence must be versioned.

## 4. Candidate classification

External candidates shall be classified as:

- `EXT-A` — strong baseline match;
- `EXT-B` — acceptable with measured covariate imbalance;
- `EXT-C` — contextual reference only;
- `REJECTED` — material mismatch or unstable management/access.

These classes are evidence states, not quality scores.

No candidate may be promoted after outcome inspection merely because it produces a preferred treatment contrast.

## 5. Preferred design

Where feasible, use more than one neighboring reference plot rather than relying on a single external plot.

Rationale:

- neighboring plots may differ in soil history, management or drainage;
- a multi-reference design reduces dependence on one idiosyncratic comparator;
- it allows sensitivity analysis across references.

The number of external reference plots is not yet frozen because candidate availability, plot area, sampling-unit dimensions, variance and access remain unknown.

## 6. Spatial geometry requirements

Each external reference requires:

- stable comparator ID;
- closed polygon;
- local or geographic coordinates with declared CRS;
- area;
- distance and direction from Genesis;
- topographic position;
- access points;
- excluded subareas;
- sampling strata;
- geometry evidence source;
- geometry hash/version.

A point marker or owner description alone is insufficient.

## 7. Baseline matching variables

Before confirmatory comparison, collect the same baseline variables in Genesis and each external candidate:

- primary infiltration endpoint;
- aggregate stability;
- SOC concentration;
- bulk density if SOC stock is used;
- soil moisture at measurement;
- soil texture/profile evidence;
- vegetation/ground cover;
- slope/aspect/topographic position;
- disturbance/management history;
- rainfall/weather context where material.

Matching decisions must be recorded before treatment-effect interpretation.

## 8. Analysis architecture

The preferred estimand is a prespecified treatment-versus-reference difference in change where repeated pre/post data support it.

Where multiple external references are retained:

- report individual-reference contrasts;
- report a prespecified pooled/weighted contrast only if the weighting rule is frozen in advance;
- retain heterogeneity rather than averaging it away silently.

The design must account for repeated measures and spatial clustering.

Subsamples within one field do not become independent field-level replicates by repetition.

## 9. Bias-control rules

The following are prohibited:

- selecting the neighboring field after seeing treatment outcomes;
- changing comparator because another field produces a larger effect;
- using different measurement methods in Genesis and comparator;
- ignoring management changes on the comparator;
- calling a materially unmatched neighboring parcel equivalent;
- presenting historical baseline alone as a contemporaneous control.

All deviations must be logged.

## 10. Access / owner record

Before baseline collection, each external candidate must have a documented access state:

- owner/operator identified;
- sampling permission state;
- permitted activities;
- access window;
- disturbance constraints;
- contact/reference record;
- any compensation or relationship relevant to COI/disclosure.

No ownership right is inferred from proximity.

## 11. Promotion gate to P1.3M

P1.3L-EXT becomes **FROZEN** only when:

1. at least one external candidate polygon is defined;
2. access is secured;
3. baseline comparability variables are collected;
4. management history is sufficiently documented;
5. candidate classification is recorded;
6. primary comparator set is selected before outcome interpretation;
7. reviewer/COI scope is assigned.

Until then, external-reference route is selected but not complete.

## 12. Claim boundary

Selection of a neighboring comparator does not establish ecological efficacy, causal effect, field validation, certification, legal/market admission or PRU/TRBK value.
