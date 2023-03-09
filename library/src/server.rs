pub use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tokio::signal;

use crate::{dl, git};

async fn shutdown() {
    signal::ctrl_c().await.unwrap()
}

fn setup() -> Router {
    let app = Router::new()
        .nest("/dl", dl::setup())
        .nest("/index", git::smarthttp::setup())
        .route("/test", get(handler));

    app
}

#[tokio::main]
pub async fn main() {
    // build our application with a route
    let app = setup();

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    let shutdown = server.with_graceful_shutdown(shutdown());

    if let Err(e) = shutdown.await {
        eprintln!("{e}");
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
