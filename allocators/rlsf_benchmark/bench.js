import init, { run_bench } from "./pkg/rlsf_benchmark.js";
await init(Deno.readFile("./pkg/rlsf_benchmark_bg.wasm"));
run_bench();
