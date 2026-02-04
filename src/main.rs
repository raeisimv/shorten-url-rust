use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::shortener::shortener_handler;
mod shortener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // adjust tracing and log
    let tracing_opt = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into());
    tracing_subscriber::registry().with(tracing_opt).init();

    tracing::debug!("initializing the web app ...");
    let app = get_router();
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let addr = format!("127.0.0.1:{}", port);
    let sock = tokio::net::TcpListener::bind(addr).await?;
    println!(
        "server is listening on http://{}",
        sock.local_addr().unwrap()
    );
    let _serv = axum::serve(sock, app).await?;

    tracing::info!("done! exiting the app...");
    Ok(())
}

fn get_router() -> Router {
    Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/shorten", post(shortener_handler))
}
