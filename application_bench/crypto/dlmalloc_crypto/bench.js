import init, { run_bench } from "./pkg/dlmalloc_crypto.js";
await init(Deno.readFile("./pkg/dlmalloc_crypto_bg.wasm"));
run_bench();
