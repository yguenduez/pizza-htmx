mod handlers;
mod model;
mod templates;

use axum::{routing::get, Router};
use handlers::{get_todos, handler, post_todo};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos).post(post_todo));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
