use matrix_blog::run;
use std::net::SocketAddr;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Starting matrix-blog on {}", addr);

    if let Err(e) = run(addr).await {
        error!("Failed to start matrix-blog: {}", e);
        std::process::exit(1);
    }
}
