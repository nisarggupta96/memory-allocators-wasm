// default dlmalloc

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_bench() {
    let mut vec = Vec::with_capacity(100);
    vec.extend(0..300usize);
    log("Hello, world!, dlmalloc");
}
