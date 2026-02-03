use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // adjust tracing and log
    let tracing_opt = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into());
    tracing_subscriber::registry().with(tracing_opt).init();

    info!("done! exiting the app...");
    Ok(())
}
