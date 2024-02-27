use axum::extract;
use axum::http::{header, HeaderMap, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn file_pool(
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
