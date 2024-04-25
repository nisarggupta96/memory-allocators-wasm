import init, { run_bench } from "./pkg/buddy_alloc_benchmark.js";
await init(Deno.readFile("./pkg/buddy_alloc_benchmark_bg.wasm"));
run_bench();
