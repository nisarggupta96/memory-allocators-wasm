import init, { run_bench } from "./pkg/talc_benchmark.js";
await init(Deno.readFile("./pkg/talc_benchmark_bg.wasm"));
run_bench();
