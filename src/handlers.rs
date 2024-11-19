use axum::response::IntoResponse;

use crate::templates::HelloTemplate;

pub async fn handler() -> impl IntoResponse {
    HelloTemplate { name: "Hans" }
}
