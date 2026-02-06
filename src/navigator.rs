use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;

pub async fn navigator_handler(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<String, (StatusCode, String)> {
    Ok(code)
}
