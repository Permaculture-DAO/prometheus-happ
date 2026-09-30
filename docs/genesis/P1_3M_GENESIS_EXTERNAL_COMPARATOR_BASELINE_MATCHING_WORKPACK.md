# PROMETHEUS P1.3M — Genesis External Comparator Baseline Matching Workpack

Status: **ACTIVE WORKPACK — CANDIDATE NEIGHBOR PARCELS/plots NOT YET IDENTIFIED**  
Date: 2026-09-30

## 1. Purpose

Operationalize P1.3L-EXT by defining the field data and decision table required to select one or more neighboring external reference plots before confirmatory outcome interpretation.

## 2. Candidate register

For every neighboring candidate create one record containing:

- `candidate_id`;
- owner/operator reference;
- cadastral/locational reference if available;
- polygon geometry and CRS;
- usable area;
- distance from Genesis;
- access status;
- management status;
- known prior disturbance;
- hydrological relationship to Genesis;
- notes/evidence references.

No candidate is accepted merely because it is adjacent.

## 3. Matching matrix

Each candidate shall be compared with Genesis on:

| Matching domain | Required evidence | Admission rule |
|---|---|---|
| soil/profile | field description and/or lab evidence | material mismatch disclosed; critical mismatch rejects |
| parent material/geology | source evidence where available | major mismatch rejects for soil endpoints |
| elevation/topographic position | measured or verified spatial data | comparable band required |
| slope | field/GIS/survey measurement | material difference quantified |
| aspect/exposure | measured/derived | required where outcome-relevant |
| hydrologic position | field mapping | must be independent from Genesis treatment spillover |
| land-use history | owner/history/source records | recent major intervention may reject |
| vegetation/ground cover | standardized baseline observation | material differences quantified |
| management stability | owner/operator agreement + ledger | unstable management may reject |
| access continuity | documented permission | repeated measurement must be feasible |

## 4. Baseline endpoint package

The same baseline package must be collected in Genesis and each surviving external candidate.

### PE-01 infiltration
Freeze:
- apparatus/method;
- water head or fixed volume;
- antecedent-moisture rule;
- replicate rule;
- unit;
- operator QA;
- weather/disturbance record.

### PE-02 aggregate stability
Freeze:
- sample depth;
- preparation;
- method/sieve;
- replicate/composite rule;
- unit;
- lab/field QA.

### PE-03 SOC
Freeze:
- depth increment(s);
- sampling/compositing rule;
- laboratory method;
- geolocation/resampling rule;
- bulk density if stock is claimed.

Companion observations:
- soil moisture;
- ground cover;
- soil texture/profile;
- topographic position;
- management/disturbance state.

## 5. Candidate decision states

After baseline matching:

- `EXT-A`: retained primary candidate — strong baseline match;
- `EXT-B`: retained with modeled/measured covariate imbalance;
- `EXT-C`: contextual reference only;
- `REJECTED`: unacceptable mismatch, spillover risk, unstable management or insufficient access.

Classification must occur before treatment-outcome inspection.

## 6. Multi-reference architecture

Preferred if feasible:

- at least two external candidate plots reaching EXT-A or EXT-B;
- preserve individual plot identity in analysis;
- do not pool automatically;
- freeze any pooled/weighted estimator before outcome inspection.

The exact number remains open until landowner access and candidate availability are known.

## 7. Sampling-frame rule

Sampling coordinates are generated only after candidate polygons and strata are frozen.

For each accepted comparator:

1. exclude boundaries/access/disturbed zones;
2. stratify by topography/soil/cover where needed;
3. define independent sampling units;
4. determine replicate count from precision/power or an explicit pilot rationale;
5. freeze coordinates;
6. collect baseline using identical SOPs.

## 8. Timing rule

Comparator baseline should be collected as close in time as practical to the Genesis baseline and under comparable seasonal/antecedent conditions.

If timing differs materially:

- record the difference;
- retain weather/soil-moisture context;
- constrain inference accordingly.

## 9. Management-change protocol

Any comparator management change during the observation window must create a disturbance record including:

- date;
- operation;
- affected geometry;
- input/material;
- intensity if known;
- expected relevance to primary endpoints.

A material management change may downgrade or disqualify the comparator for a given endpoint.

## 10. Selection freeze record

When comparator selection is complete create:

`GENESIS_EXTERNAL_COMPARATOR_FREEZE_<version>`

containing:

- accepted comparator IDs;
- polygon hashes;
- baseline matching table;
- access evidence state;
- management-history state;
- candidate classification;
- primary/secondary comparator role;
- frozen endpoint/SOP versions;
- reviewer/COI record;
- allowed claim scope.

## 11. Immediate blocking facts

P1.3M cannot progress to geometry freeze until the following are known:

1. which neighboring owners/plots are available;
2. their exact location/polygon;
3. whether repeated sampling is permitted;
4. whether their management will remain stable;
5. enough baseline information to test comparability.

## 12. Claim boundary

P1.3M defines comparator-selection operations only. It does not establish ecological efficacy, causal superiority, certification, legal/market admission or PRU/TRBK value.
