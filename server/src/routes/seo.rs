use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;

use crate::content::ContentLoader;

fn get_base_url() -> String {
    std::env::var("RUST_BASE_URL")
        .map(|url| format!("https://{}", url))
        .unwrap_or_else(|_| "https://example.com".to_string())
}

pub async fn handle_robots() -> impl IntoResponse {
    let base_url = get_base_url();
    let content = format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        base_url
    );
    (
        StatusCode::OK,
        [("Content-Type", "text/plain")],
        content,
    )
}

pub async fn handle_sitemap(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    let posts = loader.get_all_posts().await;
    let base_url = get_base_url();

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);

    xml.push_str(&format!(
        r#"  <url>
    <loc>{}/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#,
        base_url
    ));

    for post in &posts {
        let date = post.frontmatter.date.format("%Y-%m-%d").to_string();
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}/posts/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
"#,
            base_url, post.slug, date
        ));
    }

    let pages = [("about", "2024-01-01")];
    for (slug, date) in pages {
        xml.push_str(&format!(
            r#"  <url>
    <loc>{}/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.6</priority>
  </url>
"#,
            base_url, slug, date
        ));
    }

    xml.push_str("</urlset>");

    (
        StatusCode::OK,
        [("Content-Type", "application/xml")],
        xml,
    )
}

pub async fn handle_feed(State(loader): State<Arc<ContentLoader>>) -> impl IntoResponse {
    let posts = loader.get_all_posts().await;
    let base_url = get_base_url();

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\" xmlns:content=\"http://purl.org/rss/1.0/modules/content/\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str("    <title>Matrix Blog</title>\n");
    xml.push_str(&format!("    <link>{}</link>\n", base_url));
    xml.push_str("    <description>A modern, sleek, dark-mode-native blog built with Rust</description>\n");
    xml.push_str("    <language>en</language>\n");
    xml.push_str(&format!("    <atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\"/>\n", base_url));

    if let Some(first) = posts.first() {
        xml.push_str(&format!("    <lastBuildDate>{}</lastBuildDate>\n", first.frontmatter.date.to_rfc3339()));
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
            post.frontmatter.title,
            url,
            url,
            pub_date,
            desc,
            post.html
        ));
    }

    xml.push_str("  </channel>\n</rss>");

    (
        StatusCode::OK,
        [("Content-Type", "application/rss+xml")],
        xml
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
    xml.push_str(&format!("  <link href=\"{}/\" rel=\"alternate\"/>\n", base_url));
    xml.push_str(&format!("  <link href=\"{}/feed.xml\" rel=\"self\"/>\n", base_url));
    xml.push_str(&format!("  <id>{}/</id>\n", base_url));
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
            post.frontmatter.title,
            url,
            url,
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
