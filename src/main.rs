mod get_website;
mod short_url;
use crate::get_website::get_website;
use crate::short_url::short_url;
use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};
use dotenv::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let app = Router::new()
        .route("/{short_url}", get(get_website))
        .route("/shorten", post(short_url))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
