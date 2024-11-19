use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative
pub struct HelloTemplate<'a> {
    // the name of the struct can be anything
    pub name: &'a str, // the field name should match the variable name
                       // in your template
}
