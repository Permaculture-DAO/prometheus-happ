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


## WSL production-boundary observation and independent CI build — 2026-10-22

On authorized Federica, Windows confirmed `Prometheus-Holochain` is WSL2 and its registered base path is on **D:**. Pre-start free space: C ~3.962 GiB, D ~1.737 GiB. Explicit user instruction was to start the distro without altering production services.

Using `wsl.exe --distribution Prometheus-Holochain --exec /bin/sh -c '...read-only diagnostics...'`, the distro booted successfully; `wsl.exe --list --verbose` returned **Running**. `/etc/wsl.conf` declares `[boot] systemd=true`. **After boot**, a read-only `systemctl list-units --state=running` showed four PROMETHEUS units active: `prometheus-conductor`, `prometheus-bridge`, `prometheus-adapter`, `prometheus-ingress`. These were automatically started by systemd as part of distro boot. **No manual start, stop, restart, enable/disable, reconfiguration, deployment or write commands against production services were issued.** The startup itself has a lifecycle effect; do not claim the boot leaves production services entirely untouched.

Local read-only socket observations were loopback-oriented. Post-boot host free space: C ~3.918 GiB; D ~1.706 GiB. Production service state remains outside TEST-only MRF release authority. No synthetic agent, conductor, or test DNA was injected into the running distro.

On the experimental branch, a new independent GitHub Actions job `coordinator-wasm-compile` was added under the isolated MRF workflow to build `zome_coordinator_lab` on an ephemeral Ubuntu runner. **The job's result must be recorded independently when complete; publication of a workflow is not a build PASS.**

**Next gate:** compile and separately review the TEST-only read-by-ActionHash endpoint, then package a different versioned lab DNA and run bounded A/B retrieval in an ephemeral standalone conductor setup. NEVER point the experimental test at the above running PROMETHEUS services or change the deployed signed genesis DNA.
