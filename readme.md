### Memory Allocators for WebAssembly

WebAssembly (Wasm) is a portable binary code format de- signed for modern web browsers. It provides a way to run code written in multiple languages like C/C++, Go, and Rust on the web at near-native speed. Wasm delivers performance, portability, and safety, paving the way for some complex ap- plications to run on the browser, which was not possible previ- ously. WebAssembly System Interface (WASI) provides a set of portable, modular, runtime-independent, and WebAssem- bly native APIs to interact with the outside world. A wasm module can access a single "linear memory," essentially a flat array of bytes. This memory can be grown by a multiple of the page size (64K) but cannot be shrunk. WebAssembly modules can implement their custom allocators on top of the linear memory provided by the host. In this project, we study different memory allocators used widely in WebAssembly and evaluate their performance under varied workloads. Being a great language for WebAssembly, we selected Rust for our project because of its excellent tooling and customization capabilities. We compare multiple memory allocators present as Rust crates such as rlsf, wee_alloc, dlmalloc, and talc, and compare their performance for random actions and realistic workloads.

#### Chosen Memory Allocators

1. dlmalloc
2. wee_alloc
3. talc
4. rlsf
5. buddy_alloc

#### Experiments

1. Random Actions Benchmark
2. Throughput Benchmark (criterion.rs)
3. Applications
   3.1 Web Server
   3.2 Image Processing
   3.3 Cryptography

#### Execution

Install wasm-pack by following the guide - https://rustwasm.github.io/wasm-pack/

```
cd allocators
./build_and_test.sh
```

This script will run the random actions benchmark and criterion throughput benchmark. The results will be populated in the following format:

-   `allocators/output/{memory_allocator}/{criterion_throughput_results}.txt`

-   `allocators/output/{memory_allocator}/{random_actions_results}.txt`

```
cd application_bench
./build_and_test.sh
```

This script will run the application benchmarks - web server, image processing and cryptography. The results will be populated in the following format:

-   `application_bench/output/webserver/{memory_allocator}_results.txt`

-   `application_bench/output/crytpo/{memory_allocator}_results.txt`

-   `application_bench/output/image/{memory_allocator}_results.txt`
