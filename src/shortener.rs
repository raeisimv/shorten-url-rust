use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct ShortenerRequest {
    pub url: String,
    pub tag: Option<String>,
    pub ttl: Option<u32>,
}

#[derive(Serialize, Debug)]
pub struct ShortenerResponse {
    pub url: String,
}

#[derive(FromRow)]
pub struct UrlModel {
    pub id: i32,
    pub short_url: String,
    pub url: String,
    pub tag: Option<String>,
    pub ttl: Option<u32>,
}

pub async fn shortener_handler(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ShortenerRequest>,
) -> Result<Json<ShortenerResponse>, (StatusCode, String)> {
    let shorten_url = shorten(&input.url);

    let response = ShortenerResponse { url: shorten_url };
    Ok(Json(response))
}

fn shorten(url: &str) -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    hasher.finish().to_string()
}
