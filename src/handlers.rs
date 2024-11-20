use axum::response::IntoResponse;

use crate::templates::{TodoItem, TodoList, TodoPage};

pub async fn get_todos() -> impl IntoResponse {
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

    TodoList { todos }
}

pub async fn handler() -> impl IntoResponse {
    TodoPage { name: "Hans" }
}
