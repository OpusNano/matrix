use super::parser::{Page, Post};
use std::collections::HashMap;

#[derive(Default)]
pub struct ContentCache {
    pub posts: Vec<Post>,
    pub pages: Vec<Page>,
    pub page_map: HashMap<String, usize>,
    pub post_map: HashMap<String, usize>,
}

impl ContentCache {
    pub fn invalidate(&mut self) {
        self.posts.clear();
        self.pages.clear();
        self.page_map.clear();
        self.post_map.clear();
    }
}
