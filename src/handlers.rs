use crate::model::PizzaType;
use crate::pizza_storage::SqliteAdapter;
use crate::templates::{PizzaCount, PizzaItemOwned, PizzaListOwned};
use crate::{model::PostPizzaItem, templates::PizzaPage};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Form};
use std::collections::HashMap;

pub async fn get_pizzas(State(pizza_store): State<SqliteAdapter>) -> impl IntoResponse {
    let pizzas = pizza_store
        .get_pizzas()
        .await
        .into_iter()
        .map(PizzaItemOwned::from)
        .collect();
    PizzaListOwned { pizzas }
}

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn delete_pizza(
    State(pizza_store): State<SqliteAdapter>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    pizza_store.delete_pizza(id).await;

    StatusCode::NO_CONTENT
}

pub async fn post_pizza(
    State(pizza_store): State<SqliteAdapter>,
    Form(todo): Form<PostPizzaItem>,
) -> impl IntoResponse {
    let pizza_type = PizzaType::from(todo.pizza_type);
    let inserted_pizza = pizza_store.insert_pizza(todo.name, pizza_type).await;

    let owned_pizza: PizzaItemOwned = inserted_pizza.into();
    owned_pizza
}

pub async fn pizza_count(State(pizza_store): State<SqliteAdapter>) -> impl IntoResponse {
    let pizzas = pizza_store.get_pizzas().await;
    let pizzas_count: HashMap<PizzaType, u64> =
        pizzas
            .into_iter()
            .fold(HashMap::new(), |mut acc, pizza_item| {
                *acc.entry(pizza_item.pizza_type).or_insert(0) += 1;
                acc
            });

    PizzaCount {
        vegan_count: *pizzas_count.get(&PizzaType::Vegan).unwrap_or(&0u64),
        vegi_count: *pizzas_count.get(&PizzaType::Vegitarian).unwrap_or(&0u64),
        meat_count: *pizzas_count.get(&PizzaType::Meat).unwrap_or(&0u64),
    }
}

pub async fn main_page() -> impl IntoResponse {
    PizzaPage {}
}
