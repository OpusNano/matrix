use crate::content::cache::ContentCache;
use crate::content::parser::{Page, Post};
use crate::watcher::FileWatcher;
use parking_lot::RwLock;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ContentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Duplicate slug '{0}'")]
    DuplicateSlug(String),
}

pub struct ContentLoader {
    cache: Arc<RwLock<ContentCache>>,
    content_path: PathBuf,
    reload_lock: Arc<Mutex<()>>,
    _watcher: FileWatcher,
}

impl ContentLoader {
    pub fn new(content_path: PathBuf) -> Arc<Self> {
        let cache = Arc::new(RwLock::new(ContentCache::default()));
        let cache_clone = Arc::clone(&cache);
        let reload_lock = Arc::new(Mutex::new(()));

        let _watcher = FileWatcher::new(content_path.clone(), move || {
            info!("Content changed, invalidating cache");
            let mut cache = cache_clone.write();
            cache.invalidate();
        });

        let loader = Arc::new(Self {
            cache: Arc::clone(&cache),
            content_path,
            reload_lock,
            _watcher,
        });

        // Initial load
        let loader_clone = Arc::clone(&loader);
        tokio::spawn(async move {
            if let Err(e) = loader_clone.reload().await {
                error!("Initial content load failed: {}", e);
            }
        });

        loader
    }

    fn check_duplicate_slugs(posts: &[Post], pages: &[Page]) -> Result<(), ContentError> {
        let mut seen: HashSet<String> = HashSet::new();
        let mut duplicate_slug: Option<String> = None;

        for post in posts {
            if !seen.insert(post.slug.clone()) {
                duplicate_slug = Some(post.slug.clone());
                break;
            }
        }

        if duplicate_slug.is_none() {
            for page in pages {
                if !seen.insert(page.slug.clone()) {
                    duplicate_slug = Some(page.slug.clone());
                    break;
                }
            }
        }

        if let Some(slug) = duplicate_slug {
            return Err(ContentError::DuplicateSlug(slug));
        }

        Ok(())
    }

    async fn reload(&self) -> Result<(), ContentError> {
        let _lock = self.reload_lock.lock().await;

        info!("Reloading content from disk");

        let posts_dir = self.content_path.join("posts");
        let pages_dir = self.content_path.join("pages");

        let mut posts = Vec::new();
        let mut pages = Vec::new();

        // Load posts
        if posts_dir.exists() {
            for entry in walkdir::WalkDir::new(&posts_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            {
                match crate::content::parser::parse_post_file(entry.path()) {
                    Ok(post) => {
                        if !post.frontmatter.draft {
                            posts.push(post);
                        }
                    }
                    Err(e) => warn!("Failed to parse {:?}: {}", entry.path(), e),
                }
            }
        }

        // Load pages
        if pages_dir.exists() {
            for entry in walkdir::WalkDir::new(&pages_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            {
                match crate::content::parser::parse_markdown_file(entry.path()) {
                    Ok(page) => pages.push(page),
                    Err(e) => warn!("Failed to parse {:?}: {}", entry.path(), e),
                }
            }
        }

        // Check for duplicate slugs
        if let Err(e) = Self::check_duplicate_slugs(&posts, &pages) {
            error!("Duplicate slug detected: {}", e);
            return Err(e);
        }

        // Sort posts by date (newest first)
        posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

        let mut cache = self.cache.write();
        cache.posts = posts;
        cache.pages = pages;

        info!(
            "Loaded {} posts and {} pages",
            cache.posts.len(),
            cache.pages.len()
        );

        Ok(())
    }

    pub async fn get_all_posts(&self) -> Vec<Post> {
        // Ensure content is loaded
        if self.cache.read().posts.is_empty() {
            let _ = self.reload().await;
        }
        self.cache.read().posts.clone()
    }

    pub async fn get_post(&self, slug: &str) -> Option<Post> {
        if self.cache.read().posts.is_empty() {
            let _ = self.reload().await;
        }
        self.cache
            .read()
            .posts
            .iter()
            .find(|p| p.slug == slug)
            .cloned()
    }

    pub async fn get_page(&self, slug: &str) -> Option<Page> {
        if self.cache.read().pages.is_empty() {
            let _ = self.reload().await;
        }
        self.cache
            .read()
            .pages
            .iter()
            .find(|p| p.slug == slug)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::parser::Frontmatter;
    use chrono::Utc;

    fn make_post(slug: &str) -> Post {
        Post {
            slug: slug.to_string(),
            frontmatter: Frontmatter {
                title: slug.to_string(),
                date: Utc::now(),
                tags: vec![],
                draft: false,
                description: None,
            },
            html: String::new(),
        }
    }

    fn make_page(slug: &str) -> Page {
        Page {
            slug: slug.to_string(),
            frontmatter: Frontmatter {
                title: slug.to_string(),
                date: Utc::now(),
                tags: vec![],
                draft: false,
                description: None,
            },
            html: String::new(),
        }
    }

    #[test]
    fn test_check_duplicate_slugs_no_duplicates() {
        let posts = vec![make_post("post1"), make_post("post2")];
        let pages = vec![make_page("page1")];
        let result = ContentLoader::check_duplicate_slugs(&posts, &pages);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_duplicate_slugs_post_duplicate() {
        let posts = vec![make_post("same-slug"), make_post("same-slug")];
        let pages = vec![];
        let result = ContentLoader::check_duplicate_slugs(&posts, &pages);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ContentError::DuplicateSlug(s) if s == "same-slug"));
    }

    #[test]
    fn test_check_duplicate_slugs_page_duplicate() {
        let posts = vec![make_post("post1")];
        let pages = vec![make_page("same-slug"), make_page("same-slug")];
        let result = ContentLoader::check_duplicate_slugs(&posts, &pages);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_duplicate_slugs_post_page_collision() {
        let posts = vec![make_post("collision")];
        let pages = vec![make_page("collision")];
        let result = ContentLoader::check_duplicate_slugs(&posts, &pages);
        assert!(result.is_err());
    }
}
