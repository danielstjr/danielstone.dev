#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod frontend;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![frontend::home])
        .attach(Template::fairing())
}