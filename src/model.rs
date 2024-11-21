use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostPizzaItem {
    pub name: String,
    pub pizza_type: String,
}

enum PizzaType {
    Vegi,
    Meat,
    Vegan,
}

pub struct PizzaItem {
    name: String,
    pizza_type: PizzaType,
}
