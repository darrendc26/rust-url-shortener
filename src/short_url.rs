use axum::{Json, extract::State, http::StatusCode};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Debug)]
pub struct ShortenRequest {
    url: String,
}

#[derive(Serialize, Debug)]
pub struct ShortenResponse {
    id: i32,
    short_url: String,
}

pub async fn short_url(
    State(pool): State<PgPool>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), (StatusCode, Json<serde_json::Value>)> {
    let url = payload.url;

    println!("Received URL: {}", url);

    let shortened = nanoid!(7);

    let result = sqlx::query!(
        "INSERT INTO urls (url, short_url) VALUES ($1, $2) RETURNING id, short_url",
        url,
        shortened
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse {
            id: result.id,
            short_url: result.short_url,
        }),
    ))
}
