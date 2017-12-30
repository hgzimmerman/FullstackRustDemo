use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

/// Permit access to files that live in www/build/
/// The rank 2 in the macro allows other ROUTES to match first.
#[get("/<file..>", rank=10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/static";
    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY).join(file)).ok()
}

#[get("/js/<file..>", rank=9)]
pub fn js(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/target/wasm32-unknown-unknown/release";
    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY).join(file)).ok()
}

#[get("/js/app.js", rank=8)]
pub fn app() -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/target/wasm32-unknown-unknown/release/weekend_at_joes_4_frontend.js";
//    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY)).ok()
}

#[get("/weekend_at_joes_4_frontend.wasm", rank=8)]
pub fn wasm() -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/target/wasm32-unknown-unknown/release/weekend_at_joes_4_frontend.wasm";
    //    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY)).ok()
}
