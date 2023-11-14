use axum::extract;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::Html;
use axum::routing::get;
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(handler))
        .route("/client/:file_name", get(client));

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

async fn client(
    extract::Path(file_name): extract::Path<String>,
) -> Result<(HeaderMap, String), StatusCode> {
    let head_path = String::from("src/client/");
    let file_name = head_path + &file_name[..];
    let file_path = std::path::Path::new(&*file_name);
    let Some(file_extension) = file_path.extension().and_then(|x| x.to_str()) else {
        return Err(StatusCode::NOT_FOUND);
    };

    let Ok(mut file) = File::open(file_path).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let mut headers = HeaderMap::new();
    match file_extension {
        "html" => headers.insert(header::CONTENT_TYPE, "text/html".parse().unwrap()),
        "css" => headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap()),
        "js" => headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap()),
        _ => return Err(StatusCode::NOT_FOUND),
    };

    let mut body = String::new();
    if let Err(_) = file.read_to_string(&mut body).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((headers, body))
}
