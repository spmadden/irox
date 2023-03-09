use std::io::Write;

use axum::{
    body::{self, Body, BoxBody, Full, HttpBody},
    http::{self, Request},
    response::Response,
    routing::{get, post},
    RequestExt, Router,
};
use bytes::{Buf, BufMut, Bytes};

async fn upload_pack(mut request: Request<Body>) -> Response<Full<Bytes>> {
    let (inparts, inbody) = request.into_parts();
    let inbytes = hyper::body::to_bytes(inbody).await;

    match inbytes {
        Ok(b) => println!("{b:?}"),
        Err(e) => eprintln!("{e:?}"),
    }

    let mut bytes = Vec::new();
    bytes.put_slice("001e# service=git-upload-pack\n0000".as_bytes());

    let body = Full::new(bytes.into());
    let resp = Response::builder()
        .header(
            "Content-Type",
            "application/x-git-upload-pack-advertizement",
        )
        .status(200)
        .body(body)
        .unwrap();
    resp
}

pub fn setup() -> Router {
    Router::new().route("/info/refs", get(upload_pack))
}


