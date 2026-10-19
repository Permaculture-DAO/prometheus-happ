// Native Rust vs actual Node.js WebAssembly execution; synthetic toy data only.
// Run: node wasm_parity_runner.cjs <WASM> <NATIVE_VECTOR_BINARY>
const fs = require("node:fs");
const cp = require("node:child_process");
const crypto = require("node:crypto");
const assert = require("node:assert/strict");
async function main() {
  if(process.argv.length !== 4) throw new Error("Usage: node wasm_parity_runner.cjs <wasm> <native_bin>");
  const wasmPath=process.argv[2], binPath=process.argv[3];
  const bytes=fs.readFileSync(wasmPath);
  const digest=crypto.createHash("sha256").update(bytes).digest("hex");
  const module = await WebAssembly.compile(bytes);
  const imports=WebAssembly.Module.imports(module);
  if(imports.length) throw new Error("Unexpected external imports: "+JSON.stringify(imports));
  const instance=await WebAssembly.instantiate(module, {});
  const y0=instance.exports.prometheus_mrf_lab_y0;
  const y1=instance.exports.prometheus_mrf_lab_y1;
  assert.equal(typeof y0,"function");
  assert.equal(typeof y1,"function");
  const run=cp.spawnSync(binPath,[],{encoding:"utf8",maxBuffer:4*1024*1024});
  if(run.error) throw run.error;
  if(run.status!==0) throw new Error("native binary failed: "+run.stderr);
  const lines=run.stdout.trim().split(/\r?\n/);
  let compared=0;
  for(const line of lines) {
    const p=line.trim().split(",");
    if(p.length !== 4)throw new Error("malformed native vector: "+line);
    const [x0,x1,expect0,expect1] = p.map(BigInt);
    const actual0 = y0(x0,x1);
    const actual1 = y1(x0,x1);
    assert.equal(actual0,expect0,"y0 mismatch on "+x0+","+x1);
    assert.equal(actual1,expect1,"y1 mismatch on "+x0+","+x1);
    compared++;
  }
  assert.equal(compared,4108,"unexpected vector count");
  console.log(JSON.stringify({
    status:"PASS",type:"synthetic_native_wasm_runtime_parity",
    vectors:compared,scalar_comparisons:compared*2,
    node:process.version,wasm_bytes:bytes.length,wasm_sha256:digest,
    wasm_imports:imports.length,native_exit_code:run.status
  },null,2));
}
main().catch(e=>{console.error(e.stack||e);process.exit(1);});