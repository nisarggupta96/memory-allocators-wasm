// buddy_alloc
use std::alloc::Layout;
use wasm_bindgen::prelude::*;

use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

const FAST_HEAP_SIZE: usize = 32 * 1024; // 32 KB
const HEAP_SIZE: usize = 1024 * 1024; // 1M
const LEAF_SIZE: usize = 16;

pub static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
pub static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe {
    let fast_param = FastAllocParam::new(FAST_HEAP.as_ptr(), FAST_HEAP_SIZE);
    let buddy_param = BuddyAllocParam::new(HEAP.as_ptr(), HEAP_SIZE, LEAF_SIZE);
    NonThreadsafeAlloc::new(fast_param, buddy_param)
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_bench() {
    let mut vec = Vec::with_capacity(100);
    vec.extend(0..300usize);
    log("Hello, world! buddy_alloc");
    execute_bench();
}

const ACTIONS: usize = 100000;
const ITERATIONS: usize = 100;

pub fn execute_bench() {
    console_error_panic_hook::set_once();

    let timer = web_sys::window().unwrap().performance().unwrap();

    // warm up
    random_actions();

    // go!
    let start = timer.now();
    for _ in 0..ITERATIONS {
        random_actions();
    }
    let end = timer.now();

    // log durations
    let total_ms = end - start;
    let average_ms = total_ms / ITERATIONS as f64;
    let apms = ACTIONS as f64 / average_ms / 1000.0;
    log(format!("  total time: {} ms", total_ms).as_str());
    log(format!("  average time for {} actions: {} ms", ACTIONS, average_ms).as_str());
    log(format!("  average actions/us: {:.1}", apms).as_str());
}

fn random_actions() {
    let mut score = 0;
    let mut v = Vec::with_capacity(10000);

    while score < ACTIONS {
        let action = fastrand::usize(0..3);

        match action {
            0 => {
                let size = fastrand::usize(100..=1000);
                let align = 8 << fastrand::u16(..).trailing_zeros() / 2;
                let layout = Layout::from_size_align(size, align).unwrap();

                let allocation = unsafe { std::alloc::alloc(layout) };

                if !allocation.is_null() {
                    v.push((allocation, layout));
                    score += 1;
                }
            }
            1 => {
                if !v.is_empty() {
                    let index = fastrand::usize(0..v.len());
                    let (ptr, layout) = v.swap_remove(index);

                    unsafe {
                        std::alloc::dealloc(ptr, layout);
                    }

                    score += 1;
                }
            }
            2 => {
                if !v.is_empty() {
                    let index = fastrand::usize(0..v.len());
                    if let Some((ptr, layout)) = v.get_mut(index) {
                        let new_size = fastrand::usize(100..=10000);

                        unsafe {
                            let realloc = std::alloc::realloc(*ptr, *layout, new_size);

                            if !realloc.is_null() {
                                *ptr = realloc;
                                *layout =
                                    Layout::from_size_align_unchecked(new_size, layout.align());
                                score += 1;
                            }
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    for (ptr, layout) in v {
        unsafe {
            std::alloc::dealloc(ptr, layout);
        }
    }
}
