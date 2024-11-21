use axum::{response::IntoResponse, Form};

use crate::templates::PizzaItemOwned;
use crate::{
    model::PostPizzaItem,
    templates::{PizzaItem, PizzaList, TodoPage},
};

pub async fn get_todos() -> impl IntoResponse {
    let todos = vec![
        PizzaItem {
            name: "Bernd",
            pizza_type: "Vegi",
        },
        PizzaItem {
            name: "Hans",
            pizza_type: "Meaty",
        },
    ];

    PizzaList { pizzas: todos }
}

pub async fn post_todo(Form(todo): Form<PostPizzaItem>) -> impl IntoResponse {
    PizzaItemOwned {
        name: todo.name,
        pizza_type: todo.pizza_type,
    }
}

pub async fn handler() -> impl IntoResponse {
    TodoPage { name: "Hans" }
}
