use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::content::ContentLoader;
use crate::routes::{
    handle_feed, handle_feed_xml, handle_index, handle_page, handle_post, handle_robots,
    handle_sitemap, handle_tag, handle_tags_index,
};

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

async fn handle_about(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    handle_page(Path("about".to_string()), State(loader)).await
}

fn get_mime_type(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("html") => "text/html",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("eot") => "application/vnd.ms-fontobject",
        _ => "application/octet-stream",
    }
}

async fn handle_static(Path(path): Path<String>) -> impl IntoResponse {
    let static_dir = std::path::Path::new("static");

    let clean_path = path
        .split('/')
        .filter(|&segment| !segment.is_empty() && segment != "..")
        .collect::<Vec<_>>()
        .join("/");

    if clean_path.is_empty() || clean_path.contains("..") {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }

    let file_path = static_dir.join(&clean_path);

    let canonical_static = match static_dir.canonicalize() {
        Ok(p) => p,
        Err(_) => return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let canonical_file = match file_path.canonicalize() {
        Ok(p) => p,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return axum::http::StatusCode::NOT_FOUND.into_response()
        }
        Err(_) => return axum::http::StatusCode::NOT_FOUND.into_response(),
    };

    if !canonical_file.starts_with(&canonical_static) {
        return axum::http::StatusCode::FORBIDDEN.into_response();
    }

    let mime = get_mime_type(&canonical_file);

    match tokio::fs::read(&canonical_file).await {
        Ok(contents) => {
            let mut response = axum::response::Response::new(contents.into());
            response
                .headers_mut()
                .insert(axum::http::header::CONTENT_TYPE, mime.parse().unwrap());
            response.headers_mut().insert(
                axum::http::header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );
            response.headers_mut().insert(
                axum::http::header::X_CONTENT_TYPE_OPTIONS,
                "nosniff".parse().unwrap(),
            );
            response
        }
        Err(_) => axum::http::StatusCode::NOT_FOUND.into_response(),
    }
}
