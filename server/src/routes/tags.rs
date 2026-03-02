use axum::{
    extract::Path,
    extract::State,
    response::Html,
    http::StatusCode,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::content::{ContentLoader, Post};
use crate::templates::render_tags_index;

fn get_base_url() -> String {
    std::env::var("RUST_BASE_URL")
        .map(|url| format!("https://{}", url))
        .unwrap_or_else(|_| "https://example.com".to_string())
}

pub async fn handle_tags_index(State(loader): State<Arc<ContentLoader>>) -> Html<String> {
    let posts = loader.get_all_posts().await;

    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for post in &posts {
        for tag in &post.frontmatter.tags {
            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    let mut tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1));

    Html(render_tags_index(&tags))
}

pub async fn handle_tag(
    Path(tag): Path<String>,
    State(loader): State<Arc<ContentLoader>>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let posts = loader.get_all_posts().await;

    let filtered: Vec<Post> = posts
        .into_iter()
        .filter(|p| p.frontmatter.tags.contains(&tag))
        .collect();

    if filtered.is_empty() {
        return Err((StatusCode::NOT_FOUND, "Tag not found"));
    }

    Ok(Html(render_tag_page(&tag, &filtered)))
}

fn render_tag_page(tag: &str, posts: &[Post]) -> String {
    let base_url = get_base_url();
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("    <meta name=\"description\" content=\"Posts tagged with {}\">\n", tag));
    html.push_str(&format!("    <title>Tag: {} - Matrix Blog</title>\n", tag));
    html.push_str(&format!(
        "    <link rel=\"canonical\" href=\"{}/tags/{}\">\n",
        base_url, tag
    ));
    html.push_str("    <meta property=\"og:type\" content=\"website\">\n");
    html.push_str(&format!("    <meta property=\"og:url\" content=\"{}/tags/{}\">\n", base_url, tag));
    html.push_str(&format!("    <meta property=\"og:title\" content=\"Tag: {}\">\n", tag));
    html.push_str(&format!("    <meta property=\"og:description\" content=\"Posts tagged with {}\">\n", tag));
    html.push_str("    <meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str(&format!("    <meta name=\"twitter:title\" content=\"Tag: {}\">\n", tag));
    html.push_str("    <link rel=\"stylesheet\" href=\"/static/css/style.css\">\n");
    html.push_str("    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/static/favicon.svg\">\n");
    html.push_str("    <script src=\"/static/js/main.js\" defer></script>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("    <a href=\"#main-content\" class=\"skip-link\">Skip to content</a>\n");
    html.push_str("    \n");
    html.push_str("    <header class=\"site-header\">\n");
    html.push_str("        <nav class=\"site-nav\" role=\"navigation\" aria-label=\"Main navigation\">\n");
    html.push_str("            <a href=\"/\" class=\"site-logo\">Matrix</a>\n");
    html.push_str("            <ul class=\"nav-links\">\n");
    html.push_str("                <li><a href=\"/\">Home</a></li>\n");
    html.push_str("                <li><a href=\"/about\">About</a></li>\n");
    html.push_str("                <li><a href=\"/tags\">Tags</a></li>\n");
    html.push_str("            </ul>\n");
    html.push_str("        </nav>\n");
    html.push_str("    </header>\n");
    html.push_str("    \n");
    html.push_str("    <main id=\"main-content\" class=\"main-content\">\n");
    html.push_str("        <div class=\"container\">\n");
    html.push_str("            <section class=\"posts-section\">\n");
    html.push_str(&format!("                <h1 class=\"page-title\">Tag: {}</h1>\n", tag));
    html.push_str("                \n");
    html.push_str("                <ul class=\"post-list\">\n");

    for post in posts {
        let date = post.frontmatter.date.format("%Y-%m-%d").to_string();

        let tags_html: Vec<String> = post
            .frontmatter
            .tags
            .iter()
            .map(|t| {
                if t == tag {
                    format!("<span class=\"tag\">{}</span>", t)
                } else {
                    format!("<a href=\"/tags/{}\" class=\"tag\">{}</a>", t, t)
                }
            })
            .collect();

        let desc_html = post
            .frontmatter
            .description
            .as_ref()
            .map(|d| format!("<p class=\"post-description\">{}</p>", d))
            .unwrap_or_default();

        html.push_str(&format!(
            "                    <li class=\"post-item\">\n\
                        <article class=\"post-preview\">\n\
                            <h2 class=\"post-title\">\n\
                                <a href=\"/posts/{}\">{}</a>\n\
                            </h2>\n\
                            <span class=\"post-meta\">{} &middot; {}</span>\n\
                            {}\n\
                        </article>\n\
                    </li>\n",
            post.slug, post.frontmatter.title, date, tags_html.join(" "), desc_html
        ));
    }

    html.push_str("                </ul>\n");
    html.push_str("            </section>\n");
    html.push_str("        </div>\n");
    html.push_str("    </main>\n");
    html.push_str("    \n");
    html.push_str("    <footer class=\"site-footer\">\n");
    html.push_str("        <p>&copy; 2026 Matrix Blog. Built with Rust.</p>\n");
    html.push_str("    </footer>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}
