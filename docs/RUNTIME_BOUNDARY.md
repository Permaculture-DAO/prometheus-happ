# Prometheus hApp — Runtime Boundary

This repository implements bounded Holochain-native pilot runtime behavior for Prometheus v1.1.

It does not define canonical meaning.

The canonical source of truth is:

Permaculture-DAO/prometheus-canon

Runtime code must not independently create or imply PRU issuance, RAP inclusion, token rights, investor rights, land rights, ecological credits, governance authority, public financial representations, or automatic MRV validation.

## Candidate conductor receipt — 2026-10-02 (unsigned engineering evidence)

Integrity/coordinator source: `4852a2b`, with host-test/lockfile follow-up only.
No integrity or coordinator wasm changed during the test-harness correction.
Toolchain: rustc/cargo 1.96.0; packer `hc` 0.6.1; sweettest conductor locked to
Holochain 0.6.1. The bundle is local and ignored by Git; this receipt is not a
signed release or a live runtime migration.

| Artifact | Identity |
|---|---|
| `dnas/hearth/hearth.dna` SHA256 | `9b322cdd82adbfad52b58b47d940c48dad2c026bd730a375845088d4496a160e` |
| Holochain DNA hash (`hc dna hash`) | `uhC0kxCfeiVy4g1N4uKBiDMfeSoFFcgEh7yWC3gemM9Sf6WJTIX5J` |
| Integrity wasm SHA256 | `9815acb9ad67ecc8539cb205706514804bc2c605258085a1d4b9d72bde506c27` |
| Coordinator wasm SHA256 | `05b4ca7cd45b158d139de85c2529b14f797280804d7dc2fca5c4df55f7fa5ff7` |
| Host-test lockfile SHA256 | `07a5f5010ff7a900e75bb4797b5f0eb13bbd80bf8f11245b9863089f0f530509` |

Results against this exact bundle:

- Initial full run: valid OHE PASS, Active-without-baseline rejection PASS, REAL
  gate-message rejection PASS. TEST persistence test FAILED while decoding the
  binary ActionHash as `serde_json::Value` (3 passed / 1 failed, 718.27 seconds).
- Corrected host harness uses typed ActionHash responses. Targeted `evidence`
  run: 2/2 PASS (335.24 seconds), with TEST record read-back preserving sensor_id
  and evidence_class, repeated delivery returning the same action with
  created=false, and REAL rejected with REAL_DATA_PERSISTENCE_GATE_CLOSED.
- The two unchanged OHE tests were not repeated in the targeted run. This is
  coverage across two runs, not a claimed single final 4/4 suite run.

Reproduce from this checkout after build/pack:

```bash
cargo test --locked --manifest-path integration_tests/Cargo.toml --test conductor -- --test-threads=1
```

The conductor REAL test exercises the coordinator rejection path; the integrity
REAL rule has separate 27/27 unit-test evidence, including its five gate tests.
This receipt does not claim an adversarial replacement-coordinator conductor
test. TEST fixture persistence establishes neither real-data admission nor
scientific admissibility. REAL remains NO-GO pending the authorization,
calibration-registry and separate independent-attestation workflow in #30.
