use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use rocket::Request;
use log;
use rocket::response::status;
use rocket::http::Status;


/// This will take any 404 response (that isn't explicitly returned via a normal handler),
/// and respond with the index.html file.
/// This allows for users to visit the site at any arbitrary URL and still be directed to the
/// proper route in the frontend, because responding this way doesn't alter the url,
/// or indicate any error has occurred.
///
/// This does require that the links found in index.html use absolute paths (start with "/"), because
/// otherwise they will start requesting /frontend/specific/path/js/app.js instead of just /js/app.js
/// like the backend expects if you load the page with that url.
#[error(404)]
pub fn index_from_404(req: &Request) -> status::Custom<NamedFile> {
    const WEB_DIRECTORY: &'static str = "../frontend/app/static/index.html";
    let index = NamedFile::open(Path::new(WEB_DIRECTORY))
        .unwrap();
    status::Custom(Status::Ok, index)
}

/// Permit access to files that live in the frontend's build directory
/// The rank 2 in the macro allows other ROUTES to match first.
#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "../frontend/app/static";
    log::info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY).join(file))
        .ok()
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
