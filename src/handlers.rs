use axum::{response::IntoResponse, Form};

use crate::{
    model::PostTodoItem,
    templates::{TodoItem, TodoItemCreated, TodoList, TodoPage},
};

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

pub async fn post_todo(Form(todo): Form<PostTodoItem>) -> impl IntoResponse {
    TodoItemCreated {
        name: todo.name,
        due_date: todo.due_date,
    }
}

pub async fn handler() -> impl IntoResponse {
    TodoPage { name: "Hans" }
}
