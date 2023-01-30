#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod frontend;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![frontend::home, frontend::experiences])
        .attach(Template::fairing())
}