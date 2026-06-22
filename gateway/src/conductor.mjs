// conductor.mjs — integration boundary to the local Holochain conductor.
// Calls the coordinator `create_evidence` zome function via @holochain/client.
// In --dry-run mode it does not connect (so the pipeline is testable offline).
//
// Local-first / sovereign: this runs ON the gateway next to a running conductor.
// In standalone mode PROMETHEUS_ADMIN_PORT is supplied by the runtime harness.
// No public write endpoint is exposed.

import { RuntimeClient } from "./runtime_client.mjs";

const APP_ID = process.env.PROMETHEUS_APP_ID || "prometheus";
const ROLE = process.env.PROMETHEUS_ROLE || "hearth";
const ZOME = "zome_coordinator";

let _appWs = null;
let _runtime = null;

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

async function launcherCall(fnName, payload) {
  const client = await appWs();
  const appInfo = await client.appInfo();
  const cells = appInfo?.cell_info?.[ROLE];
  if (!Array.isArray(cells) || cells.length === 0) {
    throw new Error(`conductor: role "${ROLE}" not found in app "${APP_ID}" — fail closed`);
  }
  const selected = cells.find((cell) => cell.type === "provisioned") || cells[0];
  const cell = selected.value || selected;
  if (!cell?.cell_id) throw new Error(`conductor: no cell_id for role "${ROLE}" — fail closed`);
  return client.callZome({
    cell_id: cell.cell_id,
    zome_name: ZOME,
    fn_name: fnName,
    payload,
  });
}

export async function callRuntime(fnName, payload) {
  const adminPort = Number(process.env.PROMETHEUS_ADMIN_PORT || 0);
  if (!adminPort) return launcherCall(fnName, payload);
  if (!_runtime) {
    _runtime = new RuntimeClient({
      adminPort,
      appPort: Number(process.env.PROMETHEUS_APP_PORT || 0),
      appId: APP_ID,
      role: ROLE,
      zome: ZOME,
      origin: process.env.PROMETHEUS_WS_ORIGIN || "http://localhost",
    });
  }
  return _runtime.call(fnName, payload);
}

/**
 * Submit one MRV evidence payload. Returns the ActionHash (real) or a marker (dry run).
 * @param {object} payload  MrvEvidenceEntry from buildEvidence()
 * @param {object} [o]      { dryRun }
 */
export async function submitEvidence(payload, { dryRun = false } = {}) {
  if (dryRun) {
    return { dryRun: true, would_call: `${ZOME}.create_evidence_idempotent`, payload, created: true };
  }
  const result = await callRuntime("create_evidence_idempotent", payload);
  return {
    actionHash: result.action_hash,
    created: result.created,
  };
}

export async function close() {
  if (_appWs) { await _appWs.client.close(); _appWs = null; }
  if (_runtime) { await _runtime.close(); _runtime = null; }
}
