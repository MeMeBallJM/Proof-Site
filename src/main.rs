use axum::response::Html;
use axum::routing::get;
use std::net::SocketAddr;

mod utils;
use utils::data_routing;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(handler))
        .route("/client/:file_name", get(data_routing::file_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn handler() -> Html<&'static str> {
    Html(include_str!("./client/index.html"))
}
