# Release Manifest — `v1.1.3-runtime-proof` (CANDIDATE — UNSIGNED)

> **Status: CANDIDATE, not signed, not released, not ratified.** This is a **runtime**
> release (Holochain happ/dna + gateway), NOT a canon amendment: the governing prose
> canon remains **v1.1.2-genesis** (white paper unchanged). Becomes a release only when
> the artefacts + this manifest are GPG-signed by the maintainer (key `8C8CB0D4`) and,
> per governance, ratified by active stewards. No silent change.

## Identity
- Release id: **v1.1.3-runtime-proof**
- Type: runtime (happ/dna/gateway). Prose canon of record: **v1.1.2-genesis** (unchanged).
- Source: `Permaculture-DAO/prometheus-happ` commit **`4679409c57f0524e83f3663fcdcd6df775189e6c`**.
- Date prepared: 2026-06-23.

## Controlling artefacts (SHA256 — from the WP4 evidence, committed & reviewed)
| Artefact | SHA256 |
|---|---|
| `hearth_prometheus.happ` | `a485337506bae6ea605c77e497fba429b52a240d4b6312e1d49f3819cce3de9b` |
| `dnas/hearth/hearth.dna` | `e18b4de1034c810af71a5a6ca06b44d5cec4791989b6bf969c1c9e6025932b55` |

- Holochain DNA hash: `uhC0kL9KUdHi368UryFN3Y9hrHJN1bQLV3U9YtLgRE4SWrpuhGuuH`
- The `.webhapp` (operator console UI wrapper) is **not** part of the runtime-proof
  scope; it is rebuilt from source (`ui/` + `hc web-app pack`) and may be signed
  separately. Only the happ + dna above are the controlling, WP4-proven artefacts.

## Toolchain (pinned; reproduce on this to verify)
holochain **0.6.1** · hc **0.6.1** · lair-keystore **0.6.3** · node **v20.19.4** ·
`@holochain/client` **0.20.5** · mqtt **5.15.1** (gateway deps lockfile-pinned).

## Evidence
Bounded **synthetic** vertical proof **WP4 B0–B8** (MQTT→gateway→Holochain, persistent
idempotency, gateway + conductor restart, second-agent matching record):
`prometheus-ops-docs/audits/runtime/v1.1.3/runs/WP4-20260622T164351Z/`.

## Immutable baseline inputs (referenced, never rebuilt here)
- Signed canon release `v1.1.2-genesis` → canon tag commit `dcafb0c8…`.
- Public registry `prometheus-canonical-releases` (CURRENT_RELEASE=v1.1.2-genesis).

## Signing (maintainer action — see `SIGNING_INSTRUCTIONS.md`)
At signing: generate `SHA256SUMS.txt` over the controlling artefacts, GPG detached-sign
each artefact and this manifest with key `8C8CB0D48C7F60DA`, verify, then publish to the
public registry under `releases/v1.1.3-runtime-proof/`.
