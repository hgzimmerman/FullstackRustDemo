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
pub fn wasm(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/target/wasm32-unknown-unknown/release";
    info!("Getting file: {}", file.to_str().unwrap());
    NamedFile::open(Path::new(WEB_DIRECTORY).join(file)).ok()
}