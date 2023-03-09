use axum::Router;

use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;

#[derive(TypedPath, Deserialize)]
#[typed_path("/:name/:version/download")]
pub struct DownloadParams {
    name: String,

    version: String,
}

async fn download_get(params: DownloadParams, _body: String) {
    println!("get: {}/{}", &params.name, &params.version);
}

async fn download_head(params: DownloadParams, _body: String) {
    println!("head: {}/{}", &params.name, &params.version);
}

pub fn setup() -> Router {
    let dl = Router::new()
        .typed_get(download_get)
        .typed_head(download_head);

    dl
}
