import init, { run_bench } from "./pkg/dlmalloc_image.js";
await init(Deno.readFile("./pkg/dlmalloc_image_bg.wasm"));
run_bench();
