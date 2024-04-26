mod handler;
mod model;
mod response;

// use buddy_alloc::{BuddyAllocParam, FastAllocParam, NonThreadsafeAlloc};

// const FAST_HEAP_SIZE: usize = 32 * 1024 * 1024; // 32 KB
// const HEAP_SIZE: usize = 64 * 1024 * 1024; // 1M
// const LEAF_SIZE: usize = 16;

// pub static mut FAST_HEAP: [u8; FAST_HEAP_SIZE] = [0u8; FAST_HEAP_SIZE];
// pub static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

// #[global_allocator]
// static ALLOC: NonThreadsafeAlloc = unsafe {
//     let fast_param = FastAllocParam::new(FAST_HEAP.as_ptr(), FAST_HEAP_SIZE);
//     let buddy_param = BuddyAllocParam::new(HEAP.as_ptr(), HEAP_SIZE, LEAF_SIZE);
//     NonThreadsafeAlloc::new(fast_param, buddy_param)
// };

// #[global_allocator]
// static A: rlsf::SmallGlobalTlsf = rlsf::SmallGlobalTlsf::new();

// use talc::*;

// static mut ARENA: [u8; 1000000] = [0; 1000000];

// #[global_allocator]
// static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> = Talc::new(unsafe {
//     // if we're in a hosted environment, the Rust runtime may allocate before
//     // main() is called, so we need to initialize the arena automatically
//     ClaimOnOom::new(Span::from_const_array(core::ptr::addr_of!(ARENA)))
// })
// .lock();

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use model::{QueryOptions, DB};
use warp::{http::Method, Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let db = model::todo_db();

    let todo_router = warp::path!("api" / "todos");
    let todo_router_id = warp::path!("api" / "todos" / String);

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(handler::health_checker_handler);

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let todo_routes = todo_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_todo_handler)
        .or(todo_router
            .and(warp::get())
            .and(warp::query::<QueryOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::todos_list_handler));

    let todo_routes_id = todo_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_todo_handler)
        .or(todo_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_todo_handler))
        .or(todo_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_todo_handler));

    let routes = todo_routes
        .with(cors)
        .with(warp::log("api"))
        .or(todo_routes_id)
        .or(health_checker);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
