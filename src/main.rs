mod handlers;
mod model;
mod pizza_storage;
mod templates;

use crate::handlers::delete_pizza;
use crate::pizza_storage::SqliteAdapter;
use axum::routing::delete;
use axum::{routing::get, Router};
use handlers::{get_pizzas, main_page, post_pizza};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:pizza.db").await.unwrap();
    let adapter = SqliteAdapter::new(pool);
    let app = Router::new()
        .route("/", get(main_page))
        .route("/pizzas", get(get_pizzas).post(post_pizza))
        .route("/pizzas/:id", delete(delete_pizza))
        .with_state(adapter);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
