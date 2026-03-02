use crate::content::{Post, Page};

fn get_base_url() -> String {
    std::env::var("RUST_BASE_URL")
        .map(|url| format!("https://{}", url))
        .unwrap_or_else(|_| "https://example.com".to_string())
}

pub fn render_index(posts: &[Post]) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <meta name=\"description\" content=\"Matrix Blog - Latest posts\">\n");
    html.push_str("    <title>Matrix Blog - Home</title>\n");
    html.push_str(&format!("    <link rel=\"canonical\" href=\"{}/\"/>\n", get_base_url()));
    html.push_str("    <meta property=\"og:type\" content=\"website\">\n");
    html.push_str(&format!("    <meta property=\"og:url\" content=\"{}/\"/>\n", get_base_url()));
    html.push_str("    <meta property=\"og:title\" content=\"Matrix Blog - Home\">\n");
    html.push_str("    <meta property=\"og:description\" content=\"Matrix Blog - Latest posts\">\n");
    html.push_str("    <meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str("    <meta name=\"twitter:title\" content=\"Matrix Blog - Home\">\n");
    html.push_str("    <meta name=\"twitter:description\" content=\"Matrix Blog - Latest posts\">\n");
    html.push_str("    <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Matrix Blog RSS Feed\" href=\"/feed.xml\">\n");
    html.push_str("    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/static/favicon.svg\">\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"/static/css/style.css\">\n");
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
    html.push_str("                <h1 class=\"page-title\">Latest Posts</h1>\n");
    html.push_str("                \n");
    html.push_str("                <ul class=\"post-list\">\n");

    for post in posts {
        let date = post.frontmatter.date.format("%Y-%m-%d").to_string();

        let tags_html: Vec<String> = post
            .frontmatter
            .tags
            .iter()
            .map(|t| format!("<a href=\"/tags/{}\" class=\"tag\">{}</a>", t, t))
            .collect();

        let meta_html = if post.frontmatter.tags.is_empty() {
            format!("<span class=\"post-meta\">{}</span>", date)
        } else {
            format!(
                "<span class=\"post-meta\">{} &middot; {}</span>",
                date,
                tags_html.join(", ")
            )
        };

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
                            {}\n\
                            {}\n\
                        </article>\n\
                    </li>\n",
            post.slug, post.frontmatter.title, meta_html, desc_html
        ));
    }

    html.push_str("                </ul>\n");
    html.push_str("            </section>\n");
    html.push_str("        </div>\n");
    html.push_str("    </main>\n");
    html.push_str("    \n");
    html.push_str("    <footer class=\"site-footer\">\n");
    html.push_str("        <p>&copy; 2026 Matrix Blog Built with Rust <span class=\"sep\">&middot;</span> <a href=\"/feed.xml\" class=\"footer-rss\">RSS Feed</a></p>\n");
    html.push_str("    </footer>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

pub fn render_post(post: &Post) -> String {
    let date = post.frontmatter.date.format("%Y-%m-%d").to_string();

    let tags_html: Vec<String> = post
        .frontmatter
        .tags
        .iter()
        .map(|t| format!("<a href=\"/tags/{}\" class=\"tag\">{}</a>", t, t))
        .collect();

    let description = post.frontmatter.description.as_deref().unwrap_or("");
    let canonical = format!("{}/posts/{}", get_base_url(), post.slug);

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("    <meta name=\"description\" content=\"{}\">\n", description));
    html.push_str(&format!("    <title>{} - Matrix Blog</title>\n", post.frontmatter.title));
    html.push_str(&format!("    <link rel=\"canonical\" href=\"{}\">\n", canonical));
    html.push_str("    <meta property=\"og:type\" content=\"article\">\n");
    html.push_str(&format!("    <meta property=\"og:url\" content=\"{}\">\n", canonical));
    html.push_str(&format!("    <meta property=\"og:title\" content=\"{}\">\n", post.frontmatter.title));
    html.push_str(&format!("    <meta property=\"og:description\" content=\"{}\">\n", description));
    html.push_str("    <meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str(&format!("    <meta name=\"twitter:title\" content=\"{}\">\n", post.frontmatter.title));
    html.push_str(&format!("    <meta name=\"twitter:description\" content=\"{}\">\n", description));
    if !post.frontmatter.tags.is_empty() {
        html.push_str(&format!(
            "    <meta property=\"article:tag\" content=\"{}\">\n",
            post.frontmatter.tags.join("\">\n    <meta property=\"article:tag\" content=\"")
        ));
    }
    html.push_str(&format!(
        "    <meta property=\"article:published_time\" content=\"{}\">\n",
        post.frontmatter.date.to_rfc3339()
    ));
    html.push_str("    <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Matrix Blog RSS Feed\" href=\"/feed.xml\">\n");
    html.push_str("    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/static/favicon.svg\">\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"/static/css/style.css\">\n");
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
    html.push_str("        <article class=\"post-full\">\n");
    html.push_str("            <header class=\"post-header\">\n");
    html.push_str(&format!(
        "                <h1 class=\"post-title\">{}</h1>\n",
        post.frontmatter.title
    ));
    if !tags_html.is_empty() {
        html.push_str(&format!(
            "                <span class=\"post-meta\">{} &middot; {}</span>\n",
            date,
            tags_html.join(", ")
        ));
    } else {
        html.push_str(&format!("                <span class=\"post-meta\">{}</span>\n", date));
    }
    html.push_str("            </header>\n");
    html.push_str("            \n");
    html.push_str("            <div class=\"post-content markdown-body\">\n");
    html.push_str(&post.html);
    html.push_str("\n            </div>\n");
    html.push_str("            \n");
    html.push_str("            <nav class=\"post-nav\" aria-label=\"Post navigation\">\n");
    html.push_str("                <a href=\"/\">&larr; Back to posts</a>\n");
    html.push_str("            </nav>\n");
    html.push_str("        </article>\n");
    html.push_str("    </main>\n");
    html.push_str("    \n");
    html.push_str("    <footer class=\"site-footer\">\n");
    html.push_str("        <p>&copy; 2026 Matrix Blog Built with Rust <span class=\"sep\">&middot;</span> <a href=\"/feed.xml\" class=\"footer-rss\">RSS Feed</a></p>\n");
    html.push_str("    </footer>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

pub fn render_page(page: &Page) -> String {
    let description = page.frontmatter.description.as_deref().unwrap_or("");
    let canonical = format!("{}/{}", get_base_url(), page.slug);

    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("    <meta name=\"description\" content=\"{}\">\n", description));
    html.push_str(&format!("    <title>{} - Matrix Blog</title>\n", page.frontmatter.title));
    html.push_str(&format!("    <link rel=\"canonical\" href=\"{}\">\n", canonical));
    html.push_str("    <meta property=\"og:type\" content=\"website\">\n");
    html.push_str(&format!("    <meta property=\"og:url\" content=\"{}\">\n", canonical));
    html.push_str(&format!("    <meta property=\"og:title\" content=\"{}\">\n", page.frontmatter.title));
    html.push_str(&format!("    <meta property=\"og:description\" content=\"{}\">\n", description));
    html.push_str("    <meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str(&format!("    <meta name=\"twitter:title\" content=\"{}\">\n", page.frontmatter.title));
    html.push_str(&format!("    <meta name=\"twitter:description\" content=\"{}\">\n", description));
    html.push_str("    <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Matrix Blog RSS Feed\" href=\"/feed.xml\">\n");
    html.push_str("    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/static/favicon.svg\">\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"/static/css/style.css\">\n");
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
    html.push_str("        <article class=\"page-content\">\n");
    html.push_str("            <header class=\"page-header\">\n");
    html.push_str(&format!(
        "                <h1 class=\"page-title\">{}</h1>\n",
        page.frontmatter.title
    ));
    html.push_str("            </header>\n");
    html.push_str("            \n");
    html.push_str("            <div class=\"markdown-body\">\n");
    html.push_str(&page.html);
    html.push_str("\n            </div>\n");
    html.push_str("            \n");
    html.push_str("            <nav class=\"page-nav\" aria-label=\"Page navigation\">\n");
    html.push_str("                <a href=\"/\">&larr; Back to home</a>\n");
    html.push_str("            </nav>\n");
    html.push_str("        </article>\n");
    html.push_str("    </main>\n");
    html.push_str("    \n");
    html.push_str("    <footer class=\"site-footer\">\n");
    html.push_str("        <p>&copy; 2026 Matrix Blog Built with Rust <span class=\"sep\">&middot;</span> <a href=\"/feed.xml\" class=\"footer-rss\">RSS Feed</a></p>\n");
    html.push_str("    </footer>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

pub fn render_tags_index(tags: &[(String, usize)]) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str("    <meta name=\"description\" content=\"All tags on Matrix Blog\">\n");
    html.push_str("    <title>Tags - Matrix Blog</title>\n");
    html.push_str(&format!("    <link rel=\"canonical\" href=\"{}/tags\">\n", get_base_url()));
    html.push_str("    <meta property=\"og:type\" content=\"website\">\n");
    html.push_str(&format!("    <meta property=\"og:url\" content=\"{}/tags\">\n", get_base_url()));
    html.push_str("    <meta property=\"og:title\" content=\"Tags - Matrix Blog\">\n");
    html.push_str("    <meta property=\"og:description\" content=\"All tags on Matrix Blog\">\n");
    html.push_str("    <meta name=\"twitter:card\" content=\"summary\">\n");
    html.push_str("    <meta name=\"twitter:title\" content=\"Tags - Matrix Blog\">\n");
    html.push_str("    <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Matrix Blog RSS Feed\" href=\"/feed.xml\">\n");
    html.push_str("    <link rel=\"icon\" type=\"image/svg+xml\" href=\"/static/favicon.svg\">\n");
    html.push_str("    <link rel=\"stylesheet\" href=\"/static/css/style.css\">\n");
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
    html.push_str("            <section class=\"tags-section\">\n");
    html.push_str("                <h1 class=\"page-title\">All Tags</h1>\n");
    html.push_str("                <div class=\"tag-cloud\">\n");

    for (tag, count) in tags {
        html.push_str(&format!(
            "                    <a href=\"/tags/{}\" class=\"tag-item\">{} ({})</a>\n",
            tag, tag, count
        ));
    }

    html.push_str("                </div>\n");
    html.push_str("            </section>\n");
    html.push_str("        </div>\n");
    html.push_str("    </main>\n");
    html.push_str("    \n");
    html.push_str("    <footer class=\"site-footer\">\n");
    html.push_str("        <p>&copy; 2026 Matrix Blog Built with Rust <span class=\"sep\">&middot;</span> <a href=\"/feed.xml\" class=\"footer-rss\">RSS Feed</a></p>\n");
    html.push_str("    </footer>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}
