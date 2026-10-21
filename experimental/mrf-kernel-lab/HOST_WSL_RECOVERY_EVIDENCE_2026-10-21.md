# HOST-WSL-01 — Controlled workstation recovery evidence (2026-10-21)

**Status:** Host capacity incident partially mitigated; DHT/multi-conductor proof still separately gated. This report is an operational appendix to draft MRF [PR #48](https://github.com/Permaculture-DAO/prometheus-happ/pull/48), not an authoritative Canon update, no production runtime authorization.

## Evidence observed directly on authorized workstation Federica
- Following a previous failed Holochain peer smoke (`Wsl/Service/CreateInstance/E_FAIL`, guest passwd/locale I/O errors and inability to open temporary SQLite WASM database), the **Windows host C volume had approximately 0.07 GiB free**. A second D volume had only 0.63 GiB free.
- The controlled experimental conductor `target/debug` cache (generated build data, not source) was found to occupy several GiB; removing **only** that verified, non-linked cache freed ~3.6 GiB on host C.
- A second verified `zome_integrity_lab/target/debug` cache (~0.67 GiB) was removed. All source repositories, GitHub branches, packed TEST DNAs, release WASM artifacts, evidence documents, private keys and application databases were left untouched.
- Windows reported **C: 4.397 GiB free** after these two controlled cache removals. **D: remained ~0.631 GiB free** and was not cleaned.
- Host `WSLService` and `vmcompute` services were running; both WSL v2 distributions appeared Stopped. The default Ubuntu sparse `ext4.vhdx` consumed about 95.57 GiB on C; a separate project WSL disk was ~3.92 GiB on D.
- After freeing space, `wsl.exe -d Ubuntu -e /usr/bin/true` exited **0**; a subsequent read-only `df -h` worked. The guest filesystem reported extensive *virtual* free blocks while host C was 99% full; guest free block counts are not a substitute for host free physical capacity.
- A bounded Python standard-library SQLite test used a uniquely named disposable directory under guest `/tmp`; create/insert/read succeeded, then the temporary directory was removed automatically (`sqlite_temp_directory=PASS`, `insert_and_read=PASS`, test DB 4096 bytes).

## Safety invariants
- No WSL service shutdown, `wsl --shutdown`, VHDX compaction, filesystem repair, distro export/import or process kill was performed.
- No destructive operations on shared project files, keys, databases, signed Canon, installed production DNA or active hApp were authorized or executed.
- Reclaim additional volume capacity with a separately reviewed disk inventory; never delete backups, VHDX files or unreviewed `D:\tmp` material as speculative cleanup.

## Engineering acceptance before heavy distributed tests
1. Verify stable Ubuntu launch, guest/user lookup, host/guest disk capacity and SQLite temporary DB semantics.
2. Maintain a host free-space safety margin (engineering target: >=10 GiB C and >=5 GiB D, not a Microsoft/Holochain formal requirement), or execute distributed smoke on a separately provisioned ephemeral host with sufficient capacity.
3. Run one bounded peer-info smoke from an existing compiled test binary, record its **completed** test result rather than a mid-test printed line.
4. Only then test two independently owned agents reading/verifying the same immutable receipt through the DHT, with explicit propagation timeout, canonical action hash, negative op types, network partitions, and convergence criteria.
5. Preserve separate conclusions for in-process exact-DNA 7/7 PASS, independent CI PASS, peer-info discovery, and true DHT replication. Neither prior PASS proves REAL-world evidence or oracle independence.

**Current restriction:** `REAL_DATA_PERSISTENCE_GATE_CLOSED`, production write security, scientific certification, Aeterna/Ravel/PRU/RAP/TRBK financial/token boundaries and signed Canon remain unchanged.
