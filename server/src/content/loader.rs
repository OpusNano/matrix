use crate::content::parser::{Frontmatter, Post, Page};
use crate::content::cache::ContentCache;
use crate::watcher::FileWatcher;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tracing::{info, error, warn};

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub struct ContentLoader {
    cache: Arc<RwLock<ContentCache>>,
    content_path: PathBuf,
    _watcher: FileWatcher,
}

impl ContentLoader {
    pub fn new(content_path: PathBuf) -> Arc<Self> {
        let cache = Arc::new(RwLock::new(ContentCache::default()));
        let cache_clone = Arc::clone(&cache);
        let content_path_clone = content_path.clone();
        
        let _watcher = FileWatcher::new(content_path.clone(), move || {
            info!("Content changed, invalidating cache");
            let mut cache = cache_clone.write();
            cache.invalidate();
        });
        
        let loader = Arc::new(Self {
            cache: Arc::clone(&cache),
            content_path,
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
    
    async fn reload(&self) -> Result<(), ContentError> {
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
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
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
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
            {
                match crate::content::parser::parse_markdown_file(entry.path()) {
                    Ok(page) => pages.push(page),
                    Err(e) => warn!("Failed to parse {:?}: {}", entry.path(), e),
                }
            }
        }
        
        // Sort posts by date (newest first)
        posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
        
        let mut cache = self.cache.write();
        cache.posts = posts;
        cache.pages = pages;
        
        info!("Loaded {} posts and {} pages", cache.posts.len(), cache.pages.len());
        
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
        self.cache.read().posts.iter()
            .find(|p| p.slug == slug)
            .cloned()
    }
    
    pub async fn get_page(&self, slug: &str) -> Option<Page> {
        if self.cache.read().pages.is_empty() {
            let _ = self.reload().await;
        }
        self.cache.read().pages.iter()
            .find(|p| p.slug == slug)
            .cloned()
    }
}
