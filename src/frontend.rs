use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn home() -> Template {
    Template::render("home", context! {})
}