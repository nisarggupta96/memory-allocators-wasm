// buddy_alloc
use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

const FAST_HEAP_SIZE: usize = 32 * 1024 * 1024; // 32 KB
const HEAP_SIZE: usize = 64 * 1024 * 1024;
const LEAF_SIZE: usize = 16;

pub static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
pub static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

#[global_allocator]
static ALLOC: NonThreadsafeAlloc = unsafe {
    let fast_param = FastAllocParam::new(FAST_HEAP.as_ptr(), FAST_HEAP_SIZE);
    let buddy_param = BuddyAllocParam::new(HEAP.as_ptr(), HEAP_SIZE, LEAF_SIZE);
    NonThreadsafeAlloc::new(fast_param, buddy_param)
};
