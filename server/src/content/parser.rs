use chrono::{DateTime, Utc};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Frontmatter parse error: {0}")]
    Frontmatter(String),
    #[error("Markdown parse error: {0}")]
    Markdown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub description: Option<String>,
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            date: Utc::now(),
            tags: vec![],
            draft: true,
            description: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Post {
    pub slug: String,
    pub frontmatter: Frontmatter,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub slug: String,
    pub frontmatter: Frontmatter,
    pub html: String,
}

pub fn parse_markdown_file(path: &Path) -> Result<Page, ParseError> {
    let content = fs::read_to_string(path)?;
    parse_markdown(&content, path)
}

pub fn parse_markdown(content: &str, source_path: &Path) -> Result<Page, ParseError> {
    let (frontmatter, markdown) = extract_frontmatter(content)?;

    let slug = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let markdown = strip_first_h1_from_markdown(&markdown);
    let html = render_markdown(&markdown);

    Ok(Page {
        slug,
        frontmatter,
        html,
    })
}

fn extract_frontmatter(content: &str) -> Result<(Frontmatter, String), ParseError> {
    if !content.starts_with("---") {
        let fm = Frontmatter::default();
        return Ok((fm, content.to_string()));
    }

    let rest = content.strip_prefix("---").unwrap();
    if let Some(end) = rest.find("---") {
        let yaml_str = &rest[..end];
        let markdown = rest[end + 3..].trim_start().to_string();

        let frontmatter: Frontmatter =
            serde_yaml::from_str(yaml_str).map_err(|e| ParseError::Frontmatter(e.to_string()))?;

        Ok((frontmatter, markdown))
    } else {
        let fm = Frontmatter::default();
        Ok((fm, content.to_string()))
    }
}

fn strip_first_h1_from_markdown(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();

    if lines.is_empty() {
        return markdown.to_string();
    }

    let first_line = lines[0].trim();

    if first_line.starts_with("# ") || first_line.starts_with("#\u{00A0}") {
        let mut result = String::new();
        for line in lines.iter().skip(1) {
            result.push_str(line);
            result.push('\n');
        }
        return result.trim().to_string();
    }

    markdown.to_string()
}

fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn parse_post_file(path: &Path) -> Result<Post, ParseError> {
    let content = fs::read_to_string(path)?;
    let page = parse_markdown(&content, path)?;

    Ok(Post {
        slug: page.slug,
        frontmatter: page.frontmatter,
        html: page.html,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontmatter_parsing() {
        let content = r#"---
title: Hello World
date: 2024-01-15T10:00:00Z
tags: [rust, blog]
draft: false
description: A test post
---

# Hello"#;

        let (fm, md) = extract_frontmatter(content).unwrap();
        assert_eq!(fm.title, "Hello World");
        assert!(!fm.draft);
        assert_eq!(fm.tags, vec!["rust", "blog"]);
        assert_eq!(md.trim(), "# Hello");
    }

    #[test]
    fn test_no_frontmatter() {
        let content = "# Just Markdown";
        let (fm, md) = extract_frontmatter(content).unwrap();
        assert_eq!(fm.title, "");
        assert!(fm.draft);
        assert_eq!(md, "# Just Markdown");
    }

    #[test]
    fn test_strip_first_h1() {
        let markdown = r#"# Welcome to Matrix Blog

This is a test.
## Section

More content."#;
        let result = strip_first_h1_from_markdown(markdown);
        assert!(!result.contains("# Welcome"));
        assert!(result.contains("This is a test."));
        assert!(result.contains("## Section"));
    }

    #[test]
    fn test_strip_first_h1_preserves_content() {
        let markdown = r#"# Title

Content after."#;
        let result = strip_first_h1_from_markdown(markdown);
        assert!(result.contains("Content after"));
        assert!(!result.contains("# Title"));
    }
}
