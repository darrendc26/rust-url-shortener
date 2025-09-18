use axum::{Json, http::StatusCode, response::IntoResponse};
use rand::Rng;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

pub async fn short_url(Json(payload): Json<ShortenRequest>) -> impl IntoResponse {
    let url = payload.url;

    let mut rng = rand::rng();
    let mut short_url = "".to_string();
    for _ in 0..7 {
        let string = rng.sample(rand::distr::Alphanumeric) as char;
        short_url += &string.to_string();
    }
    let shortened = format!("https://www.example.com/{}", short_url);

    (StatusCode::OK, shortened)
}
