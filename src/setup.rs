use crate::utils::MyResult;

use sqlx::{migrate::MigrateDatabase, postgres::PgPoolOptions};
use std::{env, path::Path};

pub async fn connect_to_database() -> MyResult<sqlx::PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:thePassWord@localhost:5432/shorten_url".to_string()
    });

    // ensure database
    let db_exists = sqlx::Postgres::database_exists(&database_url).await?;
    tracing::debug!("db_exists: {}", db_exists);
    if !db_exists {
        tracing::debug!("creating database ...");
        sqlx::Postgres::create_database(&database_url).await?;
        tracing::debug!("database created");
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
    tracing::info!("run migrations in: {}", &migrations_dir.to_string_lossy());
    sqlx::migrate::Migrator::new(migrations_dir)
        .await?
        .run(&pool)
        .await?;
    tracing::info!("Ok. database is up and connected");

    Ok(pool)
}
