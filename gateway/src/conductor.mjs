// conductor.mjs — integration boundary to the local Holochain conductor.
// Calls the coordinator `create_evidence` zome function via @holochain/client.
// In --dry-run mode it does not connect (so the pipeline is testable offline).
//
// Local-first / sovereign: this runs ON the gateway (e.g. the Raspberry Pi 5) next
// to a running conductor (hc-spin / hc sandbox / packaged app). No public write
// endpoint is exposed; evidence enters the agent's own source chain and syncs P2P.

const APP_ID = process.env.PROMETHEUS_APP_ID || "prometheus";
const ROLE = process.env.PROMETHEUS_ROLE || "hearth";
const ZOME = "zome_coordinator";

let _appWs = null;

async function appWs() {
  if (_appWs) return _appWs;
  // Imported lazily so the pure pipeline (and the smoke test) need no install.
  const { AppWebsocket } = await import("@holochain/client");
  _appWs = await AppWebsocket.connect({
    // In a launcher/hc-spin context the token is provided; for a standalone
    // conductor set PROMETHEUS_APP_PORT + the installed app id.
    // url: new URL(`ws://127.0.0.1:${process.env.PROMETHEUS_APP_PORT}`),
  });
  return _appWs;
}

/**
 * Submit one MRV evidence payload. Returns the ActionHash (real) or a marker (dry run).
 * @param {object} payload  MrvEvidenceEntry from buildEvidence()
 * @param {object} [o]      { dryRun }
 */
export async function submitEvidence(payload, { dryRun = false } = {}) {
  if (dryRun) {
    return { dryRun: true, would_call: `${ZOME}.create_evidence`, payload };
  }
  const client = await appWs();
  const appInfo = await client.appInfo();
  const cell = appInfo.cell_info[ROLE].find((c) => c.type === "provisioned")?.value
    || appInfo.cell_info[ROLE][0]?.value;
  const actionHash = await client.callZome({
    cell_id: cell.cell_id,
    zome_name: ZOME,
    fn_name: "create_evidence",
    payload,
  });
  return { actionHash };
}

export async function close() {
  if (_appWs) { await _appWs.client.close(); _appWs = null; }
}
