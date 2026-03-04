use axum::{extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::content::ContentLoader;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn get_base_url() -> String {
    std::env::var("RUST_BASE_URL")
        .map(|url| {
            let url = url.trim();
            if url.starts_with("https://") || url.starts_with("http://") {
                url.to_string()
            } else {
                format!("https://{url}")
            }
        })
        .unwrap_or_else(|_| "https://example.com".to_string())
}

pub async fn handle_robots() -> impl IntoResponse {
    let base_url = get_base_url();
    let content = format!("User-agent: *\nAllow: /\n\nSitemap: {base_url}/sitemap.xml\n");
    (StatusCode::OK, [("Content-Type", "text/plain")], content)
}

pub async fn handle_sitemap(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    let posts = loader.get_all_posts().await;
    let base_url = get_base_url();

    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );

    xml.push_str(&format!(
        r#"  <url>
    <loc>{}</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#,
        escape_xml(&base_url)
    ));

    for post in &posts {
        let date = post.frontmatter.date.format("%Y-%m-%d").to_string();
        let loc = format!("{}/posts/{}", base_url, post.slug);
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
            escape_xml(&loc),
            date
        ));
    }

    let pages = [("about", "2024-01-01")];
    for (slug, date) in pages {
        let loc = format!("{base_url}/{slug}");
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
  </url>
"#,
            escape_xml(&loc),
            date
        ));
    }

    xml.push_str("</urlset>");

    (StatusCode::OK, [("Content-Type", "application/xml")], xml)
}

pub async fn handle_feed(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    let posts = loader.get_all_posts().await;
    let base_url = get_base_url();

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str("    <title>Matrix Blog</title>\n");
    xml.push_str(&format!("    <link>{}</link>\n", escape_xml(&base_url)));
    xml.push_str(
        "    <description>A modern, sleek, dark-mode-native blog built with Rust</description>\n",
    );
    xml.push_str("    <language>en</language>\n");
    let self_link = format!("{base_url}/feed.xml");
    xml.push_str(&format!(
        "    <atom:link href=\"{}\" rel=\"self\" type=\"application/rss+xml\"/>\n",
        escape_xml(&self_link)
    ));

    if let Some(first) = posts.first() {
        xml.push_str(&format!(
            "    <lastBuildDate>{}</lastBuildDate>\n",
            first.frontmatter.date.to_rfc3339()
        ));
    }

    for post in posts.iter().take(20) {
        let url = format!("{}/posts/{}", base_url, post.slug);
        let desc = post.frontmatter.description.as_deref().unwrap_or("");
        let pub_date = post.frontmatter.date.to_rfc3339();

        xml.push_str(&format!(
            r#"    <item>
      <title>{}</title>
      <link>{}</link>
      <guid isPermaLink="true">{}</guid>
      <pubDate>{}</pubDate>
      <description><![CDATA[{}]]></description>
      <content:encoded><![CDATA[{}]]></content:encoded>
    </item>
"#,
            escape_xml(&post.frontmatter.title),
            escape_xml(&url),
            escape_xml(&url),
            pub_date,
            desc,
            post.html
        ));
    }

    xml.push_str("  </channel>\n</rss>");

    (
        StatusCode::OK,
        [("Content-Type", "application/rss+xml")],
        xml,
    )
}

pub async fn handle_feed_xml(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    let posts = loader.get_all_posts().await;
    let base_url = get_base_url();

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str("  <title>Matrix Blog</title>\n");
    xml.push_str("  <subtitle>A modern, sleek, dark-mode-native blog built with Rust</subtitle>\n");
    let alt_link = format!("{base_url}/");
    xml.push_str(&format!(
        "  <link href=\"{}\" rel=\"alternate\"/>\n",
        escape_xml(&alt_link)
    ));
    let self_link = format!("{base_url}/feed.xml");
    xml.push_str(&format!(
        "  <link href=\"{}\" rel=\"self\"/>\n",
        escape_xml(&self_link)
    ));
    xml.push_str(&format!("  <id>{}</id>\n", escape_xml(&alt_link)));
    xml.push_str("  <updated>");

    if let Some(first) = posts.first() {
        xml.push_str(&first.frontmatter.date.to_rfc3339());
    } else {
        xml.push_str(&chrono::Utc::now().to_rfc3339());
    }

    xml.push_str("</updated>\n");

    for post in posts.iter().take(20) {
        let url = format!("{}/posts/{}", base_url, post.slug);
        let desc = post.frontmatter.description.as_deref().unwrap_or("");

        xml.push_str(&format!(
            r#"  <entry>
    <title>{}</title>
    <link href="{}"/>
    <id>{}</id>
    <updated>{}</updated>
    <summary>{}</summary>
  </entry>
"#,
            escape_xml(&post.frontmatter.title),
            escape_xml(&url),
            escape_xml(&url),
            post.frontmatter.date.to_rfc3339(),
            desc
        ));
    }

    xml.push_str("</feed>");

    (
        StatusCode::OK,
        [("Content-Type", "application/atom+xml")],
        xml,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
        assert_eq!(escape_xml("plain"), "plain");
    }

    #[test]
    fn test_get_base_url_with_https() {
        std::env::set_var("RUST_BASE_URL", "https://example.com");
        assert_eq!(get_base_url(), "https://example.com");
        std::env::remove_var("RUST_BASE_URL");
    }

    #[test]
    fn test_get_base_url_without_scheme() {
        std::env::set_var("RUST_BASE_URL", "example.com");
        assert_eq!(get_base_url(), "https://example.com");
        std::env::remove_var("RUST_BASE_URL");
    }

    #[test]
    fn test_get_base_url_default() {
        std::env::remove_var("RUST_BASE_URL");
        assert_eq!(get_base_url(), "https://example.com");
    }
}
