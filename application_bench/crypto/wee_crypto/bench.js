import init, { run_bench } from "./pkg/wee_crypto.js";
await init(Deno.readFile("./pkg/wee_crypto_bg.wasm"));
run_bench();
