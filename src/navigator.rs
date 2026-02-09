use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Redirect,
};

use crate::{AppState, utils::internal_error};

pub async fn navigator_handler(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Redirect, (StatusCode, String)> {
    let record = sqlx::query!(
        "select long_url, tag, ttl from urls where short_url = $1",
        code
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    let Some(record) = record else {
        return Err((StatusCode::NOT_FOUND, "URL not found".to_string()));
    };
    return Ok(Redirect::to(&record.long_url));
}
