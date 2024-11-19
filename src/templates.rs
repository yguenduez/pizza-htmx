use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "todo_list.html")] // using the template in this path, relative
pub struct HelloTemplate<'a> {
    // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
    // in your template
    pub current_list: Vec<i32>,
}

pub struct TodoItem<'a> {
    pub name: &'a str,
    pub due_date: &'a str,
}
