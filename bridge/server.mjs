import express from "express";
import { execFile, spawn } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

const HOST = "127.0.0.1";
const HTTP_PORT = Number(process.env.PROMETHEUS_BRIDGE_PORT || 8787);
const HOLOCHAIN_ADMIN_PORT = Number(process.env.HOLOCHAIN_ADMIN_PORT || 14600);
const HOLOCHAIN_APP_PORT = Number(process.env.HOLOCHAIN_APP_PORT || 14602);
const HC_SANDBOX_PASSPHRASE = process.env.PROMETHEUS_HC_PASSPHRASE || "";

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
  ], { timeout: 20000 });

  return JSON.parse(stdout);
}

async function getRuntimeHealth() {
  const [apps, cells, appWebsockets] = await Promise.all([
    runHcSandboxCall("list-apps"),
    runHcSandboxCall("list-cells"),
    runHcSandboxCall("list-app-ws")
  ]);

  const appInstalled = apps.some((item) => item.installed_app_id === APP_ID);
  const dnaPresent = cells.some((cell) => cell.dna_hash === DNA_HASH);
  const appPortPresent = appWebsockets.some((ws) => ws.port === HOLOCHAIN_APP_PORT);

  return {
    appInstalled,
    dnaPresent,
    appPortPresent,
    ok: appInstalled && dnaPresent && appPortPresent
  };
}

function extractHolochainResult(rawStdout) {
  const clean = rawStdout
    .replace(/\x1B\[[0-?]*[ -/]*[@-~]/g, "")
    .replace(/hc-sandbox:\s*Enter passphrase to authorize zome calls:\s*/g, "")
    .trim();

  if (clean.includes(VERIFIED_RESULT)) {
    return VERIFIED_RESULT;
  }

  const quotedMatches = [...clean.matchAll(/"([^"]+)"/g)];
  if (quotedMatches.length > 0) {
    return quotedMatches[quotedMatches.length - 1][1].trim();
  }

  return clean
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .at(-1) || "";
}

function runHcZomeCallWithPassphrase() {
  return new Promise((resolve, reject) => {
    if (!HC_SANDBOX_PASSPHRASE) {
      reject(new Error("Missing PROMETHEUS_HC_PASSPHRASE environment variable."));
      return;
    }

    const child = spawn("hc", [
      "sandbox",
      "zome-call",
      "--running",
      String(HOLOCHAIN_ADMIN_PORT),
      APP_ID,
      DNA_HASH,
      ZOME_NAME,
      SMOKE_FUNCTION,
      "null"
    ], {
      stdio: ["pipe", "pipe", "pipe"]
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (data) => {
      stdout += data.toString();
    });

    child.stderr.on("data", (data) => {
      stderr += data.toString();
    });

    child.on("error", reject);

    child.on("close", (code) => {
      if (code !== 0) {
        reject(new Error(`hc sandbox zome-call failed with code ${code}. stderr: ${stderr.trim()} stdout: ${stdout.trim()}`));
        return;
      }

      resolve(extractHolochainResult(stdout));
    });

    child.stdin.write(`${HC_SANDBOX_PASSPHRASE}\n`);
    child.stdin.end();
  });
}

app.get("/health", async (_req, res) => {
  try {
    const runtime = await getRuntimeHealth();

    res.status(runtime.ok ? 200 : 503).json({
      status: runtime.ok ? "ok" : "degraded",
      layer: "prometheus-runtime-bridge",
      mode: "health_only_admin_api",
      app_id: APP_ID,
      holochain_admin_port: HOLOCHAIN_ADMIN_PORT,
      holochain_app_port: HOLOCHAIN_APP_PORT,
      app_installed: runtime.appInstalled,
      dna_present: runtime.dnaPresent,
      app_websocket_present: runtime.appPortPresent,
      zome: ZOME_NAME,
      smoke_function: SMOKE_FUNCTION,
      verified_runtime_result: VERIFIED_RESULT
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

app.get("/smoke-test", async (_req, res) => {
  try {
    const runtime = await getRuntimeHealth();

    if (!runtime.ok) {
      return res.status(503).json({
        status: "degraded",
        layer: "prometheus-runtime-bridge",
        mode: "zome_call_via_hc_cli_with_passphrase",
        app_id: APP_ID,
        app_installed: runtime.appInstalled,
        dna_present: runtime.dnaPresent,
        app_websocket_present: runtime.appPortPresent,
        message: "Runtime health prerequisites failed."
      });
    }

    const holochainResult = await runHcZomeCallWithPassphrase();
    const ok = holochainResult === VERIFIED_RESULT;

    res.status(ok ? 200 : 502).json({
      status: ok ? "ok" : "unexpected_result",
      layer: "prometheus-runtime-bridge",
      mode: "zome_call_via_hc_cli_with_passphrase",
      app_id: APP_ID,
      holochain_admin_port: HOLOCHAIN_ADMIN_PORT,
      holochain_app_port: HOLOCHAIN_APP_PORT,
      zome: ZOME_NAME,
      function: SMOKE_FUNCTION,
      holochain_result: holochainResult,
      expected_result: VERIFIED_RESULT
    });
  } catch (error) {
    res.status(503).json({
      status: "error",
      layer: "prometheus-runtime-bridge",
      mode: "zome_call_via_hc_cli_with_passphrase",
      app_id: APP_ID,
      holochain_admin_port: HOLOCHAIN_ADMIN_PORT,
      holochain_app_port: HOLOCHAIN_APP_PORT,
      zome: ZOME_NAME,
      function: SMOKE_FUNCTION,
      message: error?.message || String(error)
    });
  }
});

app.listen(HTTP_PORT, HOST, () => {
  console.log(`Prometheus runtime bridge listening on http://${HOST}:${HTTP_PORT}`);
  console.log(`Using Holochain admin websocket on ${HOLOCHAIN_ADMIN_PORT}`);
  console.log(`Expecting Holochain app websocket on ${HOLOCHAIN_APP_PORT}`);
  console.log(`Available endpoints: /health, /smoke-test`);
});
