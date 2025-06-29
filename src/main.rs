#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

mod frontend;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/assets", FileServer::from(relative!("assets")))
        .mount("/", routes![frontend::home, frontend::experiences])
        .attach(Template::fairing())
}