use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{AppState, utils::internal_error};

#[derive(Deserialize, Debug)]
pub struct ShortenerRequest {
    pub url: String,
    pub tag: Option<String>,
    pub ttl: Option<i64>,
}

#[derive(Serialize, Debug)]
pub struct ShortenerResponse {
    pub url: String,
}

pub async fn shortener_handler(
    State(state): State<Arc<AppState>>,
    Json(pyl): Json<ShortenerRequest>,
) -> Result<Json<ShortenerResponse>, (StatusCode, String)> {
    let shorten_url = gen_shorten_code(&pyl.url);

    // make it as short as possible
    let mut shortest = None;
    for i in 3..shorten_url.len() {
        let candidate = shorten_url[..i].to_string();
        let record = sqlx::query!(r#"select id from urls where short_url = $1"#, shorten_url)
            .fetch_optional(&state.db)
            .await
            .map_err(internal_error)?;
        if record.is_none() {
            shortest = Some(candidate);
            break;
        }
    }
    let Some(shortest) = shortest else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate short URL".to_string(),
        ));
    };

    // store it
    sqlx::query!(
        r#"
        INSERT INTO Urls (short_url, long_url, tag, ttl)
        values ($1, $2, $3, $4)
        "#,
        shortest,
        pyl.url,
        pyl.tag.unwrap_or("default".to_string()),
        pyl.ttl.unwrap_or_default()
    )
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)?;

    let response = ShortenerResponse { url: shortest };
    Ok(Json(response))
}

fn gen_shorten_code(url: &str) -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    hasher.finish().to_string()
}
