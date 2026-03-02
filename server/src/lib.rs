mod app;
mod routes;
mod content;
mod templates;
mod watcher;

pub use app::create_app;
pub use content::{ContentLoader, Post, Page};
pub use run as app_run;

use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() {
    let log_dir = std::path::PathBuf::from("logs");
    std::fs::create_dir_all(&log_dir).ok();
    
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "matrix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,matrix_blog=debug".into())
        )
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .init();
}

pub async fn run(addr: SocketAddr) {
    init_logging();
    info!("Initializing Matrix Blog");
    
    let content_path = std::path::PathBuf::from("content");
    let loader = ContentLoader::new(content_path);
    
    let app = create_app(loader);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
