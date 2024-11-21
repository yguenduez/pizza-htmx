use std::fmt::Display;

use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "todo_page.html")] // using the template in this path, relative
pub struct TodoPage<'a> {
    // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
}

#[derive(Debug)]
pub struct PizzaItem<'a> {
    pub name: &'a str,
    pub pizza_type: &'a str,
}

#[derive(Template)] // this will generate the code...
#[template(path = "todo_item_created.html")] // using the template in this path, relative
#[derive(Debug)]
pub struct PizzaItemOwned {
    pub name: String,
    pub pizza_type: String,
}

#[derive(Template)] // this will generate the code...
#[template(path = "todo_list.html")] // using the template in this path, relative
pub struct PizzaList<'a> {
    pub pizzas: Vec<PizzaItem<'a>>,
}

impl<'a> Display for PizzaItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} | {}", self.name, self.pizza_type)
    }
}
