import init, { run_app } from "./pkg/site_wasm.js";
async function main() {
  await init("/pkg/site_wasm_bg.wasm");
  run_app();
}
main();
