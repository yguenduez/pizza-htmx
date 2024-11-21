use crate::model::{PizzaItem, PizzaType};
use sqlx::{SqliteConnection, SqlitePool};

#[derive(Clone)]
pub struct SqliteAdapter {
    pool: SqlitePool,
}

impl SqliteAdapter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn insert_pizza(&self, name: String, pizza_type: PizzaType) -> PizzaItem {
        let pizza_type = pizza_type.to_string();
        sqlx::query_as!(
            PizzaItem,
            "INSERT INTO pizzas (name, pizza_type) VALUES ($1, $2) RETURNING *",
            name,
            pizza_type
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
    }

    pub async fn get_pizzas(&self) -> Vec<PizzaItem> {
        sqlx::query_as!(PizzaItem, "SELECT * FROM pizzas")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }
}
