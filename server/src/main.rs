use matrix_blog::run;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Starting matrix-blog on {}", addr);

    run(addr).await;
}
