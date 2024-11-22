use std::fmt::Display;

use crate::model;
use askama::Template;

#[derive(Template)]
#[template(path = "pizza_page.html")]
pub struct PizzaPage {}

#[derive(Debug)]
pub struct PizzaItem<'a> {
    pub name: &'a str,
    pub pizza_type: &'a str,
}

#[derive(Template)]
#[template(path = "pizza_totalize.html")]
pub struct PizzaCount {
    pub vegan_count: u64,
    pub vegi_count: u64,
    pub meat_count: u64,
}

impl From<model::PizzaItem> for PizzaItemOwned {
    fn from(value: model::PizzaItem) -> Self {
        PizzaItemOwned {
            name: value.name,
            pizza_type: value.pizza_type.to_string(),
        }
    }
}

#[derive(Template)] // this will generate the code...
#[template(path = "pizza_item_created.html")] // using the template in this path, relative
#[derive(Debug)]
pub struct PizzaItemOwned {
    pub name: String,
    pub pizza_type: String,
}

#[derive(Template)] // this will generate the code...
#[template(path = "pizza_list.html")] // using the template in this path, relative
pub struct PizzaList<'a> {
    pub pizzas: Vec<PizzaItem<'a>>,
}

#[derive(Template)] // this will generate the code...
#[template(path = "pizza_list.html")] // using the template in this path, relative
pub struct PizzaListOwned {
    pub pizzas: Vec<PizzaItemOwned>,
}

impl Display for PizzaItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} | {}", self.name, self.pizza_type)
    }
}
