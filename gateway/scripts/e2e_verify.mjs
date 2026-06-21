// e2e_verify.mjs — live end-to-end proof against a RUNNING conductor.
// Proves: (1) ingest loop works (OHE + TEST evidence + admissibility),
//         (2) the integrity validate callback REJECTS an invalid OHE live.
// All data is clearly TEST-labelled. Needs env ADMIN_PORT (+ optional APP_PORT).
import { AdminWebsocket, AppWebsocket } from "@holochain/client";
import { buildEvidence } from "../src/evidence.mjs";

const ADMIN_PORT = Number(process.env.ADMIN_PORT);
const ROLE = process.env.PROMETHEUS_ROLE || "hearth";
const ZOME = "zome_coordinator";
const ok = (m) => console.log("  OK  " + m);
const fail = (m) => { console.error("  FAIL " + m); process.exitCode = 1; };

const WSOPTS = { origin: "http://localhost" };
const admin = await AdminWebsocket.connect({ url: new URL(`ws://127.0.0.1:${ADMIN_PORT}`), wsClientOptions: WSOPTS });
const appId = process.env.PROMETHEUS_APP_ID || "prometheus";
let appPort = Number(process.env.APP_PORT);
if (!appPort) appPort = (await admin.attachAppInterface({ allowed_origins: "*" })).port;
const issued = await admin.issueAppAuthenticationToken({ installed_app_id: appId });
const cellIds = await admin.listCellIds();
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
// The cell genesis may not be integrated immediately after install; the signing-key
// grant is a source-chain commit that can transiently fail (DepMissingFromDht). Retry.
let granted = false;
for (let attempt = 1; attempt <= 8 && !granted; attempt++) {
  try { await admin.authorizeSigningCredentials(cellIds[0]); granted = true; }
  catch (e) { if (attempt === 8) throw e; await sleep(3000); }
}

const app = await AppWebsocket.connect({ url: new URL(`ws://127.0.0.1:${appPort}`), token: issued.token, wsClientOptions: WSOPTS });
const info = await app.appInfo();
const cellInfo = info.cell_info[ROLE].find((c) => c.type === "provisioned") || info.cell_info[ROLE][0];
const cell_id = (cellInfo.value || cellInfo).cell_id;
const call = (fn_name, payload) => app.callZome({ cell_id, zome_name: ZOME, fn_name, payload });

console.log("=== e2e: live conductor verification (TEST data) ===");
try {
  // 1. create a TEST OHE (Proposed)
  const subject = "TEST-ohe-sicily-1";
  await call("create_ohe", { id: subject, steward: "permaculture-dao", site: "TEST plot", status: "Proposed", baseline_recorded: false });
  ok("create_ohe (Proposed)");

  // 2. ingest 2 TEST evidence via the gateway provenance logic
  for (const r of [
    { sensor_id: "SE01-LS-TEST", indicator: "soil_moisture", observed_at: 1900000000, value: 0.31, raw: { v: 0.31 } },
    { sensor_id: "WSC2-L-TEST", indicator: "precipitation", observed_at: 1900000000, value: 12.4, raw: { mm: 12.4 } },
  ]) {
    const ev = buildEvidence(r, { subject_id: subject, calibration_hash: "calib-TEST", confidence: 0.9, reviewer: "reviewer-TEST" });
    ev.subject_id = subject; // keep one subject (buildEvidence would prefix TEST- again)
    await call("create_evidence", ev);
  }
  ok("create_evidence x2 (provenance-anchored, linked to subject)");

  // 3. assess admissibility over the subject's chain evidence
  const res = await call("assess_subject_admissibility", { subject_id: subject, legal_gate: true, confidence_threshold: 0.85, require_reviewer: true });
  if (res && res.admissible === true && res.authority_boundary === "admissibility_only_no_value") ok("assess_subject_admissibility -> admissible=true, no value");
  else fail("assess result unexpected: " + JSON.stringify(res));

  // 4. LIVE VALIDATION: an Active OHE without baseline must be REJECTED by validate
  try {
    await call("create_ohe", { id: "TEST-ohe-bad", steward: "x", site: "y", status: "Active", baseline_recorded: false });
    fail("invalid OHE (Active w/o baseline) was NOT rejected");
  } catch (e) {
    ok("invalid OHE rejected by integrity validate (live): " + String(e).slice(0, 60));
  }

  console.log(process.exitCode ? "E2E: FAIL" : "E2E: PASS");
} catch (e) {
  fail("e2e error: " + (e?.message || e));
  console.log("E2E: FAIL");
} finally {
  try { await app.client.close(); } catch {}
  try { await admin.client.close(); } catch {}
  process.exit(process.exitCode || 0);
}
