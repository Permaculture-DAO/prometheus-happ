# Holochain Runtime Design — Benchmark Intelligence Layer

## Target entry types

The Benchmark Intelligence Layer should eventually introduce the following Holochain records:

- EvaluationProtocol
- EvidenceRecord
- BenchmarkManifest
- EvaluationRun
- EvaluationScore
- EvaluationReceipt
- ReviewerAttestation
- AppealRecord

## Minimal viable runtime path

The first runtime implementation should avoid overfitting.

Phase 1 should implement:

1. create_evidence_record
2. get_evidence_record
3. create_evaluation_receipt
4. get_evaluation_receipt
5. list_my_evaluation_receipts
6. hello_benchmark_layer

## Governance boundary

Protocol scoring logic may be versioned off-chain in markdown, CSV and JSON Schema first.

Holochain should store references, evidence records, evaluation receipts and reviewer attestations.

Full automated scoring should only be introduced after protocol freeze.

## Runtime safety

Do not store private raw evidence directly in public records.

Store evidence references, hashes, metadata, access scope and review status.

Raw evidence may remain private, encrypted, external, local-first, or consent-gated.

## Benchmark integrity

Every benchmark record should include:

- protocol identifier;
- protocol version;
- subject identifier;
- evidence manifest reference;
- score components;
- decision;
- confidence;
- reviewer mode;
- authority boundary;
- timestamp.
