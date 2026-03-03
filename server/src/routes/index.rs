use axum::{extract::State, response::Html};
use std::sync::Arc;

use crate::content::ContentLoader;
use crate::templates::render_index;

pub async fn handle_index(State(loader): State<Arc<ContentLoader>>) -> Html<String> {
    let posts = loader.get_all_posts().await;
    Html(render_index(&posts))
}
