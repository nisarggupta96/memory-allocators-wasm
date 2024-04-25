import init, { run_bench } from "./pkg/dlmalloc_benchmark.js";
await init(Deno.readFile("./pkg/dlmalloc_benchmark_bg.wasm"));
run_bench();
