use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

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

pub async fn shortener_handler(
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
