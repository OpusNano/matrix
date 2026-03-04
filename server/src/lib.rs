mod app;
mod content;
mod routes;
mod templates;
mod watcher;

pub use app::create_app;
pub use content::{ContentLoader, Page, Post};
pub use run as app_run;

use std::net::SocketAddr;
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static LOG_GUARD: std::sync::OnceLock<WorkerGuard> = std::sync::OnceLock::new();

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let log_dir = std::path::PathBuf::from("logs");
    std::fs::create_dir_all(&log_dir)?;

    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "matrix.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let guard = LOG_GUARD.set(guard).ok();

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,matrix_blog=debug".into());

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_writer(non_blocking))
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .init();

    if guard.is_none() {
        eprintln!("Warning: Logging was already initialized");
    }

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error("Failed to initialize logging: {0}")]
    LoggingInit(#[from] Box<dyn std::error::Error>),
    #[error("Failed to bind to address {0}: {1}")]
    Bind(SocketAddr, std::io::Error),
    #[error("Failed to serve application: {0}")]
    Serve(#[from] std::io::Error),
}

pub async fn run(addr: SocketAddr) -> Result<(), RunError> {
    init_logging().map_err(RunError::LoggingInit)?;
    info!("Initializing Matrix Blog");

    let content_path = std::path::PathBuf::from("content");
    let loader = ContentLoader::new(content_path);

    let app = create_app(loader);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| RunError::Bind(addr, e))?;
    info!("Matrix Blog listening on {}", addr);

    axum::serve(listener, app).await.map_err(RunError::Serve)?;

    Ok(())
}
