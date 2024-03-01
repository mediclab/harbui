use anyhow::Result;
use rocket::fs::NamedFile;
use serde_json::json;
use std::path::PathBuf;

pub mod api;
mod types;

#[get("/<_path..>")]
pub async fn image(_path: PathBuf) -> Result<NamedFile, std::io::Error> {
    NamedFile::open("public/index.html").await
}

#[catch(default)]
pub fn error_handler() -> String {
    json!({ "code": "UNKNOWN", "message": "Unknown error occurred"}).to_string()
}
