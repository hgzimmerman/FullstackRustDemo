use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use warp;
use crate::routes::API_STRING;
use crate::error::Error;
use warp::path::Peek;
use warp::fs::File;

const INDEX_PATH: &str = "../../frontend/app/target/release/static/index.html";
// TODO not fully baked.
const STATIC_DIR_PATH: &str = "";
const WASM_PATH: &str = "";


/// Expose filters that work with static files
pub fn static_files_handler() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Static files handler");

    let files = index()
        .or(wasm())
        .or(index_static_file_refirect());

    warp::any()
        .and(files)
        .with(warp::log("static_files"))
        .boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_refirect() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::peek())
        .and(warp::fs::file(INDEX_PATH))
        .and_then(|segments: Peek, file: File| {
            // Reject the request if the path starts with /api/
            if let Some(first_segment) = segments.segments().next() {
                if first_segment == API_STRING {
                    return Error::NotFound.reject()
                }
            }
            Ok(file)
        })
        .boxed()
}

// TODO, not fully baked yet
fn index() ->  BoxedFilter<(impl Reply,)> {
    warp::get2()
//        .and(warp::path::index())
        .and(warp::fs::dir(STATIC_DIR_PATH))
        .boxed()
}

// TODO not fully baked
fn wasm() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::path("app.wasm"))
        .and(warp::fs::dir(WASM_PATH))
        .boxed()
}