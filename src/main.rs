#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Poscast {
    id: String,
    name: String,
    url: String,
}

#[get("/podcasts")]
fn todo() -> Json<Vec<Poscast>> {
    let mut podcast_list = Vec::new();
    podcast_list.push(Poscast {
        id: String::from("asdf234"),
        name: String::from("asdf234"),
        url: String::from("asdf234")
    });
    Json(podcast_list)
}

fn main() {
    rocket::ignite().mount("/", routes![todo]).launch();
}
