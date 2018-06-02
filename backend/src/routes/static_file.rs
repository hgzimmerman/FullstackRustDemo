use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use rocket::Request;
use log::info;
//use rocket::response::status;
//use rocket::http::Status;


use rocket_contrib::Json;

// TODO move this to /error
/// Json 404 override
#[error(404)]
pub fn json_404(_req: &Request) -> Json<String> {
    Json(
        "Could not find the requested resource"
            .to_string(),
    )
}
#[error(500)]
pub fn json_500(_req: &Request) -> Json<String> {
    Json(
        "Server encountered an internal error"
            .to_string(),
    )
}
#[error(401)]
pub fn json_401(_req: &Request) -> Json<String> {
    Json(
        "User authentication is required. You must log in for the server to accept this request.\
         It is possible that a server-restart has invalidated your existing authentication token if you provided one."
            .to_string(),
    )
}
#[error(403)]
pub fn json_403(_req: &Request) -> Json<String> {
    Json(
        "The server understood the request, but is refusing to fulfill it. Authorization will not help."
            .to_string(),
    )
}

/// Permit access to files that live in the frontend's build directory
/// The rank 10 in the macro allows other ROUTES to match first.
///
/// If the file can't be found, it will check if the requested path starts with "api".
/// If it does, then it will return a normal 404 error.
/// If it doesn't, then it assumes that the request is looking for the index, and will return that.
/// This behavior allows the frontend webapp to make get requests with the in-app URI and still recieve
/// the index and therefore the rest of the app. This will preserve the URI and allow the app to route
/// itself after it loads.
#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "../frontend/app/static";
    info!("Getting file: {}", file.to_str().unwrap());


    match NamedFile::open(Path::new(WEB_DIRECTORY).join(
        file.clone(),
    )) {
        Ok(file) => Some(file),
        Err(_) => {
            if file.starts_with("api") {
                None
            } else {
                info!("Could not find file, returning index");
                Some(index())
            }
        }
    }
}

#[get("/", rank = 10)]
pub fn index() -> NamedFile {
    info!("Getting index.html");
    const WEB_DIRECTORY: &'static str = "../frontend/app/static/index.html";
    NamedFile::open(Path::new(WEB_DIRECTORY))
        .unwrap()
}

#[get("/js/app.js", rank = 8)]
pub fn js() -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "../frontend/target/wasm32-unknown-unknown/release/app.js";
    //    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY))
        .ok()
}

#[get("/app.wasm", rank = 8)]
pub fn wasm() -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "../frontend/target/wasm32-unknown-unknown/release/app.wasm";
    //    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY))
        .ok()
}
