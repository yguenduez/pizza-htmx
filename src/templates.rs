use std::fmt::Display;

use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "todo_page.html")] // using the template in this path, relative
pub struct TodoPage<'a> {
    // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
}

#[derive(Debug)]
pub struct TodoItem<'a> {
    pub name: &'a str,
    pub due_date: &'a str,
}

#[derive(Template)] // this will generate the code...
#[template(path = "todo_item_created.html")] // using the template in this path, relative
#[derive(Debug)]
pub struct TodoItemCreated {
    pub name: String,
    pub due_date: String,
}

#[derive(Template)] // this will generate the code...
#[template(path = "todo_list.html")] // using the template in this path, relative
pub struct TodoList<'a> {
    pub todos: Vec<TodoItem<'a>>,
}

impl<'a> Display for TodoItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} | {}", self.name, self.due_date)
    }
}
