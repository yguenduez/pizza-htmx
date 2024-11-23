mod handlers;
mod model;
mod pizza_storage;
mod templates;

use crate::handlers::{delete_pizza, delete_pizza_empty, health, pizza_count};
use crate::pizza_storage::SqliteAdapter;
use axum::routing::{delete, post};
use axum::{routing::get, Router};
use dotenvy::dotenv;
use handlers::{get_pizzas, main_page, post_pizza};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let run_type = std::env::var("TYPE").unwrap();
    let ip_address = match run_type.as_str() {
        "prod" => std::env::var("SERVING_IP_PROD").unwrap(),
        _ => std::env::var("SERVING_IP_DEV").unwrap(),
    };
    let pool = SqlitePool::connect("sqlite:pizza.db").await.unwrap();
    let adapter = SqliteAdapter::new(pool);
    let app = Router::new()
        .route("/", get(main_page))
        .route("/pizzas", get(get_pizzas).post(post_pizza))
        .route("/remove/pizzas/:id", post(delete_pizza_empty))
        .route("/pizzas/:id", delete(delete_pizza))
        .route("/health", get(health))
        .route("/pizza-count", get(pizza_count))
        .with_state(adapter);

    let listener = tokio::net::TcpListener::bind(format!("{}:8080", ip_address))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
