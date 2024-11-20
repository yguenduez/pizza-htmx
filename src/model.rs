use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostTodoItem {
    pub name: String,
    pub due_date: String,
}
