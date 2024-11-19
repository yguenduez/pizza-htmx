use std::fmt::Display;

use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "todo_list.html")] // using the template in this path, relative
pub struct HelloTemplate<'a> {
    // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
    // in your template
    pub current_list: Vec<TodoItem<'a>>,
}

#[derive(Debug)]
pub struct TodoItem<'a> {
    pub name: &'a str,
    pub due_date: &'a str,
}

impl<'a> Display for TodoItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} | {}", self.name, self.due_date)
    }
}
