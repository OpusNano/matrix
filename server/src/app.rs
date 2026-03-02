use axum::{
    routing::get,
    extract::{Path, State},
    Router,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::content::ContentLoader;
use crate::routes::{handle_index, handle_page, handle_post, handle_robots, handle_sitemap, handle_feed, handle_feed_xml, handle_tags_index, handle_tag};

pub fn create_app(content_loader: Arc<ContentLoader>) -> Router {
    Router::new()
        .route("/", get(handle_index))
        .route("/about", get(handle_about))
        .route("/posts/{slug}", get(handle_post))
        .route("/tags", get(handle_tags_index))
        .route("/tags/{tag}", get(handle_tag))
        .route("/robots.txt", get(handle_robots))
        .route("/sitemap.xml", get(handle_sitemap))
        .route("/feed.xml", get(handle_feed))
        .route("/atom.xml", get(handle_feed_xml))
        .route("/static/{*path}", get(handle_static))
        .with_state(content_loader)
}

async fn handle_about(
    State(loader): State<Arc<ContentLoader>>,
) -> impl IntoResponse {
    handle_page(Path("about".to_string()), State(loader)).await
}

async fn handle_static(Path(path): Path<String>) -> impl IntoResponse {
    let static_dir = std::path::Path::new("static");
    let file_path = static_dir.join(&path);
    
    if file_path.exists() && file_path.is_file() {
        match std::fs::read(&file_path) {
            Ok(contents) => {
                let mime = match file_path.extension().and_then(|e| e.to_str()) {
                    Some("css") => "text/css",
                    Some("js") => "application/javascript",
                    Some("html") => "text/html",
                    Some("png") => "image/png",
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("svg") => "image/svg+xml",
                    _ => "application/octet-stream",
                };
                let mut response = axum::response::Response::new(contents.into());
                response.headers_mut().insert(
                    axum::http::header::CONTENT_TYPE,
                    mime.parse().unwrap(),
                );
                response.headers_mut().insert(
                    axum::http::header::CACHE_CONTROL,
                    "public, max-age=31536000, immutable".parse().unwrap(),
                );
                return response;
            }
            Err(_) => return axum::http::StatusCode::NOT_FOUND.into_response(),
        }
    }
    
    axum::http::StatusCode::NOT_FOUND.into_response()
}
