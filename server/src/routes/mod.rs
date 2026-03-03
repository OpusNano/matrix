mod index;
mod pages;
mod posts;
mod seo;
mod tags;

pub use index::handle_index;
pub use pages::handle_page;
pub use posts::handle_post;
pub use seo::{handle_feed, handle_feed_xml, handle_robots, handle_sitemap};
pub use tags::{handle_tag, handle_tags_index};
