import { copyFileSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

const [runDir] = process.argv.slice(2);
if (!runDir) {
  throw new Error("usage: node finalize_results.mjs <run-dir>");
}

const readJson = (file) => JSON.parse(readFileSync(file, "utf8"));
const corePath = path.join(runDir, "RESULTS.json");
const core = readJson(corePath);
const b7 = readJson(path.join(runDir, "B7_RESULTS.json"));
const b0 = readJson(path.join(runDir, "b0", "results.json"));

copyFileSync(corePath, path.join(runDir, "RESULTS_B1_B6_B8.json"));

const tests = {
  B0: {
    status: b0.status,
    broker: b0.broker,
    image_digest: b0.image_digest,
    payload_identical: b0.payload_identical,
    source_run_id: b0.run_id,
  },
  ...core.tests,
  B7: {
    status: b7.status,
    persisted_records: b7.persisted_records,
    gateway_survived_restart: b7.gateway_survived_restart,
    reauthorized: b7.reauthorized,
  },
};

const allPass = Object.values(tests).every((test) => test.status === "PASS");
const final = {
  schema_version: "0.2",
  generated_by: "finalize_results.mjs",
  run_id: core.run_id,
  overall_status: allPass ? "PASS" : "FAIL",
  scope: "synthetic MQTT-to-Holochain runtime proof",
  data_class: "synthetic_TEST",
  tests,
  claim_boundary: {
    admissibility: "binary_only_no_value",
    empirical_validation: false,
    production_admission: false,
  },
  known_limitations_file: "KNOWN_LIMITATIONS.md",
};

writeFileSync(corePath, `${JSON.stringify(final, null, 2)}\n`);
console.log(JSON.stringify(final, null, 2));
