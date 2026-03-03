use axum::{extract::Path, extract::State, http::StatusCode, response::Html};
use std::sync::Arc;

use crate::content::ContentLoader;
use crate::templates::render_post;

pub async fn handle_post(
    Path(slug): Path<String>,
    State(loader): State<Arc<ContentLoader>>,
) -> Result<axum::response::Html<String>, (StatusCode, &'static str)> {
    let post = loader
        .get_post(&slug)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Post not found"))?;

    Ok(Html(render_post(&post)))
}
