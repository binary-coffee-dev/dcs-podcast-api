use futures::executor::block_on;
use rocket_contrib::json::Json;
use rocket::{Route, State};

use crate::libs::database_client::DatabaseClient;
use crate::libs::models::Podcast;
use crate::libs::repositories::{insert_podcast, podcast_list, find_podcast_by_id};


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

#[post("/podcasts", format = "json", data = "<new_podcast>")]
fn create_podcast(database_client: State<DatabaseClient>, new_podcast: Json<Podcast>) -> Json<Podcast> {
    let podcast = new_podcast.into_inner();
    let new_podcast = block_on(insert_podcast(&database_client, &podcast)).expect("Fail to find the podcast by id");
    Json(new_podcast)
}

pub fn initialize_routes() -> Vec<Route> {
    routes![podcasts_list, create_podcast, get_podcast]
}
