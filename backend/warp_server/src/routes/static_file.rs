use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use warp;
use crate::routes::API_STRING;
use crate::error::Error;
use warp::path::Peek;
use warp::fs::File;

const INDEX_PATH: &str = "../../frontend/app/static/index.html";
const STATIC_DIR_PATH: &str = "../../frontend/app/static/";
const WASM_PATH: &str = "../../frontend/app/target/release/app.wasm";


/// Expose filters that work with static files
pub fn static_files_handler() -> BoxedFilter<(impl Reply,)> {
    info!("Attaching Static Files handler");

    let files = index()
        .or(wasm())
        .or(index_static_file_redirect());

    warp::any()
        .and(files)
        .with(warp::log("static_files"))
        .boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_redirect() -> BoxedFilter<(impl Reply,)> {
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
        .and(warp::path::index())
        .and(warp::fs::dir(STATIC_DIR_PATH))
        .boxed()
}

// TODO not fully baked
fn wasm() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::path("app.wasm"))
        .and(warp::path::index())
        .and(warp::fs::dir(WASM_PATH))
        .boxed()
}



#[test]
fn index_test() {
    assert!(
        warp::test::request()
            .path("/")
            .filter(&index())
            .is_ok()
    )
}

#[test]
fn static_files_404() {
    assert!(
        warp::test::request()
            .path("/api")
            .filter(&static_files_handler())
            .is_err()
    )
}

#[test]
fn static_files_redirect_to_index() {
    assert!(
        warp::test::request()
            .path("/yeet")
            .filter(&static_files_handler())
            .is_ok()
    )
}


#[test]
fn static_invalid_api_path_still_404s() {

    use crate::error::Error;
    let err = warp::test::request()
        .path("/api/yeet") // Matches nothing in the API space
        .filter(&static_files_handler());

    let err: warp::Rejection = match err {
        Ok(_) => panic!("Error was expected, found valid Reply"),
        Err(e) => e
    };
    let err = *err.into_cause::<Error>().expect("Should be a cause.");
    assert_eq!(err, Error::NotFound)

}
