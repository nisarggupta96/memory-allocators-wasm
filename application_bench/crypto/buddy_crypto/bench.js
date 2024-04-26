import init, { run_bench } from "./pkg/buddy_crypto.js";
await init(Deno.readFile("./pkg/buddy_crypto_bg.wasm"));
run_bench();
