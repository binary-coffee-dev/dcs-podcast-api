use rocket_contrib::json::Json;
use rocket::{Route, State};

use crate::database_client::DatabaseClient;
use crate::models::Podcast;

#[get("/podcasts")]
pub fn podcasts_method(database_client: State<DatabaseClient>) -> Json<Vec<Podcast>> {
    let mut podcast_list = Vec::new();
    podcast_list.push(Podcast {
        id: Some(String::from("asdf234")),
        name: Some(String::from("asdf234")),
        url: Some(String::from("asdf234")),
        banner: None
    });
    Json(podcast_list)
}

pub fn initialize_routes() -> Vec<Route> {
    routes![podcasts_method]
}
