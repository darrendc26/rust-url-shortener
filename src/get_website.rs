use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use sqlx::Row;

pub async fn get_website(
    Path(short_url): Path<String>,
    State(pool): State<sqlx::PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query("SELECT url FROM urls WHERE short_url = $1")
        .bind(short_url)
        .fetch_one(&pool)
        .await
        .unwrap();
    let url: String = result.get("url");
    println!("Redirecting to {}", url);

    Redirect::to(&url).into_response()
}
