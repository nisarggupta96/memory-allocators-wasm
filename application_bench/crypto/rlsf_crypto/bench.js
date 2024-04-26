import init, { run_bench } from "./pkg/rlsf_crypto.js";
await init(Deno.readFile("./pkg/rlsf_crypto_bg.wasm"));
run_bench();
