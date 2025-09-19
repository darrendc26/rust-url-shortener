use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

pub async fn get_website(
    Path(short_url): Path<String>,
    State(pool): State<sqlx::PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query!("SELECT url FROM urls WHERE short_url = $1", short_url)
        .fetch_one(&pool)
        .await
        .unwrap();

    let url = result.url;
    println!("Redirecting to {}", url);

    Redirect::to(&url).into_response()
}
