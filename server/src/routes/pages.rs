use axum::{extract::Path, extract::State, http::StatusCode, response::Html};
use std::sync::Arc;

use crate::content::ContentLoader;
use crate::templates::render_page;

pub async fn handle_page(
    Path(slug): Path<String>,
    State(loader): State<Arc<ContentLoader>>,
) -> Result<axum::response::Html<String>, (StatusCode, &'static str)> {
    let page = loader
        .get_page(&slug)
        .await
        .ok_or((StatusCode::NOT_FOUND, "Page not found"))?;

    Ok(Html(render_page(&page)))
}
