# Gate 5 — Stage record, 2026-10-22

The candidate branch adds an isolated TEST-only coordinator endpoint, `get_synthetic_receipt_by_action(ActionHash) -> Option<Record>`, using Holochain's `get(..., GetOptions::default())`.

**Execution status:** NOT COMPILED / NOT TESTED. This is an interface candidate, not evidence of DHT replication or a passing experiment. No conductor, runtime or DNS production configuration was changed.

## Preconditions before validation
- Review the exact Holochain 0.6.1 SDK API and compile coordinator WASM in a disposable runner with adequate storage.
- Build and hash a NEW isolated TEST DNA containing the changed coordinator; previous DNA checksums and PASS reports are NOT transferrable.
- Construct two agents with separate source chains, same experimental DNA hash, and exchange peer info.
- Agent A creates one TEST MRF receipt and captures its `ActionHash`; B MUST NOT author the same receipt.
- Agent B calls the new getter with A's action hash and checks a returned record's action hash and decoded entry against A's immutable evidence. Use bounded polling and an explicit timeout.
- Independently document DHT-validated authority outcomes. A successful `get` alone is not proof of global consensus or scientific validity.
- Negative tests: forged output, forged model hash, attempted update/delete, replay, missing dependency, partition/rejoin. Capture rejection reasons, not only `is_err()`.
- Store exact source commit, lockfile, toolchain, WASM hashes, packed DNA hash and logs before any gate advancement.

## Operational limitation
Federica measured roughly 4.16 GiB free on host C and 1.74 GiB on host D on 2026-10-22; engineering thresholds from the HOST-WSL-01 incident are unmet. Do not run a new full conductor build or alter WSL VHDX files on this host just to get a green status. Prefer an independently provisioned ephemeral CI machine.

**Boundaries:** draft/test/shadow only; signed Canon, REAL_DATA_PERSISTENCE_GATE_CLOSED, active conductor, ecological certification, PRU/RAP/TRBK and capital rights unchanged.
