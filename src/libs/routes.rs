use futures::executor::block_on;
use rocket_contrib::json::Json;
use rocket::{Route, State};

use crate::database_client::DatabaseClient;
use crate::models::Podcast;
use crate::repositories::{insert_podcast, podcast_list, find_podcast_by_id};

#[get("/podcasts")]
fn podcasts_list(database_client: State<DatabaseClient>) -> Json<Vec<Podcast>> {
    let podcast_list = block_on(podcast_list(&database_client)).expect("Fail to get the list of podcast from the database");
    Json(podcast_list)
}

#[get("/podcasts/<id>")]
fn get_podcast(database_client: State<DatabaseClient>, id: String) -> Json<Podcast> {
    let podcast = block_on(find_podcast_by_id(&database_client, &id)).expect("Fail to find the podcast by id");
    Json(podcast)
}

pub fn initialize_routes() -> Vec<Route> {
    routes![podcasts_list, get_podcast]
}
