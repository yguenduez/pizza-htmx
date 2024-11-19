use axum::response::IntoResponse;

use crate::templates::{HelloTemplate, TodoItem};

pub async fn handler() -> impl IntoResponse {
    let todos = vec![
        TodoItem {
            name: "Küche putzen",
            due_date: "Dienstag",
        },
        TodoItem {
            name: "Bad putzen",
            due_date: "Mittwoch",
        },
    ];
    HelloTemplate {
        name: "Hans",
        current_list: todos,
    }
}
