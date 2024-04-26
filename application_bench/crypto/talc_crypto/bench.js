import init, { run_bench } from "./pkg/talc_crypto.js";
await init(Deno.readFile("./pkg/talc_crypto_bg.wasm"));
run_bench();
