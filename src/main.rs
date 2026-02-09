use std::{env, path::Path};

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod navigator;
mod shortener;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // adjust tracing and log
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

async fn get_router() -> Result<Router, Box<dyn std::error::Error>> {
    use crate::navigator::navigator_handler;
    use crate::shortener::shortener_handler;

    tracing::debug!("initializing the database...");
    let app_state = app_state().await?;
    let app_state = std::sync::Arc::new(app_state);

    let app: Router = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{code}", get(navigator_handler))
        .route("/shorten", post(shortener_handler))
        .with_state(app_state);

    Ok(app)
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

async fn app_state() -> Result<AppState, Box<dyn std::error::Error>> {
    let db = connect_to_database().await?;
    Ok(AppState { db })
}

async fn connect_to_database() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:thePassWord@localhost:5432/shorten_url".to_string()
    });

    // ensure database
    let db_exists = sqlx::Postgres::database_exists(&database_url).await?;
    tracing::debug!("db_exists: {}", db_exists);
    if !db_exists {
        tracing::debug!("creating database ...");
        sqlx::Postgres::create_database(&database_url).await?;
    }

    // establish the pool connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // get migrations directory
    let migrations_dir = if env::var("RUST_ENV") == Ok("production".to_string()) {
        std::env::current_exe()?.join("./migrations")
    } else {
        let manifest_dir =
            env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env variable expected");
        Path::new(&manifest_dir).join("./migrations")
    };

    // migrate
    tracing::info!("migrating [{}]", &migrations_dir.to_string_lossy());
    sqlx::migrate::Migrator::new(migrations_dir)
        .await?
        .run(&pool)
        .await?;

    Ok(pool)
}
