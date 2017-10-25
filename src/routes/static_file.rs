use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

/// Permit access to files that live in www/build/
/// The rank 2 in the macro allows other ROUTES to match first.
#[get("/<file..>", rank=10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &'static str = "www/build";
    NamedFile::open(Path::new(WEB_DIRECTORY).join(file)).ok()
}