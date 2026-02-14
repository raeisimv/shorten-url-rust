use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::setup::connect_to_database;
use crate::utils::MyResult;

mod navigator;
mod setup;
mod shortener;
mod utils;

#[tokio::main]
async fn main() -> MyResult {
    // adjust tracing and logs
    let tracing_opt = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into());
    tracing_subscriber::registry()
        .with(tracing_opt)
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::debug!("initializing app state ...");
    let app = get_router().await?;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let addr = format!("127.0.0.1:{}", port);

    tracing::debug!("initializing the web app ...");
    let sock = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(
        "server is listening on http://{}",
        sock.local_addr().unwrap()
    );
    let _serv = axum::serve(sock, app).await?;

    tracing::info!("done! exiting the app...");
    Ok(())
}

async fn get_router() -> MyResult<Router> {
    use crate::navigator::navigator_handler;
    use crate::shortener::shortener_handler;

    tracing::debug!("initializing the database...");
    let app_state = app_state().await?;
    let app_state = std::sync::Arc::new(app_state);

    let routes = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{code}", get(navigator_handler))
        .route("/shorten", post(shortener_handler))
        .with_state(app_state);

    Ok(routes)
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

async fn app_state() -> MyResult<AppState> {
    let db = connect_to_database().await?;
    Ok(AppState { db })
}
