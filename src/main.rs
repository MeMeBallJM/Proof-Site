use axum::response::Html;
use axum::routing::get;
use std::net::SocketAddr;
use tracing::{span, Level};

mod utils;
use utils::data_routing;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = axum::Router::new().route("/client/:file", get(data_routing::fetch_file));

    let span = span!(Level::INFO, "server");
    let _enter = span.enter();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
    drop(_enter);
}
