import express from "express";
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

const HOST = "127.0.0.1";
const HTTP_PORT = Number(process.env.PROMETHEUS_BRIDGE_PORT || 8787);
const HOLOCHAIN_ADMIN_PORT = Number(process.env.HOLOCHAIN_ADMIN_PORT || 14600);
const HOLOCHAIN_APP_PORT = Number(process.env.HOLOCHAIN_APP_PORT || 14700);

const APP_ID = "hearth_prometheus";
const DNA_HASH = "uhC0kIuwnPJ1OZx6ICpBo_Qg2NrMknkLcsI-AiWANuPDgKtyMvqxf";
const ZOME_NAME = "zome_coordinator";
const SMOKE_FUNCTION = "hello_benchmark_layer";
const VERIFIED_RESULT = "Prometheus Benchmark Intelligence Layer online: evaluation_not_certification";

const app = express();
app.use(express.json());

async function runHcSandboxCall(command) {
  const { stdout } = await execFileAsync("hc", [
    "sandbox",
    "call",
    "--running",
    String(HOLOCHAIN_ADMIN_PORT),
    command
  ]);

  return JSON.parse(stdout);
}

app.get("/health", async (_req, res) => {
  try {
    const [apps, cells, appWebsockets] = await Promise.all([
      runHcSandboxCall("list-apps"),
      runHcSandboxCall("list-cells"),
      runHcSandboxCall("list-app-ws")
    ]);

    const appInstalled = apps.some((item) => item.installed_app_id === APP_ID);
    const dnaPresent = cells.some((cell) => cell.dna_hash === DNA_HASH);
    const appPortPresent = appWebsockets.some((ws) => ws.port === HOLOCHAIN_APP_PORT);

    const ok = appInstalled && dnaPresent && appPortPresent;

    res.status(ok ? 200 : 503).json({
      status: ok ? "ok" : "degraded",
      layer: "prometheus-runtime-bridge",
      mode: "health_only_admin_api",
      app_id: APP_ID,
      holochain_admin_port: HOLOCHAIN_ADMIN_PORT,
      holochain_app_port: HOLOCHAIN_APP_PORT,
      app_installed: appInstalled,
      dna_present: dnaPresent,
      app_websocket_present: appPortPresent,
      zome: ZOME_NAME,
      smoke_function: SMOKE_FUNCTION,
      verified_runtime_result: VERIFIED_RESULT,
      boundary: "This bridge health endpoint verifies the live Holochain conductor, installed hApp, cell DNA and app websocket. Direct zome-call integration will be added in the next bridge iteration."
    });
  } catch (error) {
    res.status(503).json({
      status: "error",
      layer: "prometheus-runtime-bridge",
      mode: "health_only_admin_api",
      app_id: APP_ID,
      holochain_admin_port: HOLOCHAIN_ADMIN_PORT,
      holochain_app_port: HOLOCHAIN_APP_PORT,
      message: error?.message || String(error)
    });
  }
});

app.listen(HTTP_PORT, HOST, () => {
  console.log(`Prometheus runtime bridge listening on http://${HOST}:${HTTP_PORT}`);
  console.log(`Using Holochain admin websocket on ${HOLOCHAIN_ADMIN_PORT}`);
  console.log(`Expecting Holochain app websocket on ${HOLOCHAIN_APP_PORT}`);
});
