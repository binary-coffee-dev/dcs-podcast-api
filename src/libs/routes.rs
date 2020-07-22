use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::Route;

#[derive(Serialize)]
pub struct Poscast {
    id: String,
    name: String,
    url: String,
}

#[get("/podcasts")]
pub fn podcasts_method() -> Json<Vec<Poscast>> {
    let mut podcast_list = Vec::new();
    podcast_list.push(Poscast {
        id: String::from("asdf234"),
        name: String::from("asdf234"),
        url: String::from("asdf234")
    });
    Json(podcast_list)
}

pub fn initialize_routes() -> Vec<Route> {
    routes![podcasts_method]
}
