import init, { run_bench } from "./pkg/wee_alloc_benchmark.js";
await init(Deno.readFile("./pkg/wee_alloc_benchmark_bg.wasm"));
run_bench();
