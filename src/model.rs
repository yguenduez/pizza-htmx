use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct PostPizzaItem {
    pub name: String,
    pub pizza_type: String,
}

#[derive(Serialize, Deserialize)]
pub enum PizzaType {
    Vegitarian,
    Meat,
    Vegan,
}

impl Display for PizzaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PizzaType::Vegitarian => Ok(write!(f, "Vegitarian")?),
            PizzaType::Meat => Ok(write!(f, "Meat")?),
            PizzaType::Vegan => Ok(write!(f, "Vegan")?),
        }
    }
}

impl From<String> for PizzaType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Vegitarian" => PizzaType::Vegitarian,
            "Meat" => PizzaType::Meat,
            "Vegan" => PizzaType::Vegan,
            _ => PizzaType::Meat,
        }
    }
}

#[derive(Serialize)]
pub struct PizzaItem {
    pub id: i64,
    pub name: String,
    pub pizza_type: PizzaType,
}
