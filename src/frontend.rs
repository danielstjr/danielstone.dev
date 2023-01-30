use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
struct NavItem {
    text: &'static str,
    link: &'static str,
    active: bool,
}

#[derive(Serialize)]
struct NavList {
    nav_items: Vec<NavItem>
}

#[get("/")]
pub fn home() -> Template {
    Template::render("home", &build_nav_list(0))
}

#[get("/experiences")]
pub fn experiences() -> Template {
    Template::render("experiences", &build_nav_list(1))
}

fn build_nav_list(i: usize) -> NavList {
    NavList {
        nav_items: vec![
            NavItem {
                text: "Home",
                link: "../",
                active: i == 0,
            },
            NavItem {
                text: "Experiences & Education",
                link: "../experiences",
                active: i == 1,
            },
        ]
    }
}