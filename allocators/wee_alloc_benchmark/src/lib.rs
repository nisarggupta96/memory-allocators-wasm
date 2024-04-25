// wee_alloc

use std::alloc::Layout;
use wasm_bindgen::prelude::*;
use wee_alloc::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_bench() {
    let mut vec = Vec::with_capacity(100);
    vec.extend(0..300usize);
    log("Hello, world!, wee_alloc");
}
