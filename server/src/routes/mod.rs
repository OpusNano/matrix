mod index;
mod posts;
mod pages;
mod seo;
mod tags;

pub use index::handle_index;
pub use posts::handle_post;
pub use pages::handle_page;
pub use seo::{handle_robots, handle_sitemap, handle_feed, handle_feed_xml};
pub use tags::{handle_tags_index, handle_tag};
