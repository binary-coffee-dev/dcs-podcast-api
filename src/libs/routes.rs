use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

use rocket_contrib::json::Json;
use rocket::{Route, State};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use crate::libs::database_client::DatabaseClient;
use crate::libs::models::Podcast;
use crate::libs::repositories::{insert_podcast, podcast_list, find_podcast_by_id};

use crate::libs::utils::get_env;

#[derive(Debug, Serialize, Deserialize)]
struct Jwt {
   id: String,
   role: String
}

fn is_valid_and_admin(key: &str) -> bool {
    let jwt_secret = get_env("JWT_SECRET", "THIS_IS_THE_REFERENCE");
    match String::from(key).get_mut(7..) {
        Some(jwt) => {
            match decode::<Jwt>(
                &jwt,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &Validation::new(Algorithm::HS256)
            ) {
                Ok(jwt_obj) => {
                    jwt_obj.claims.role == String::from("administrator")
                },
                Err(error) => false
            }
        },
        None => false
    }
}

pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            1 if is_valid_and_admin(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[get("/podcasts")]
pub fn podcasts_list(database_client: State<DatabaseClient>) -> Json<Vec<Podcast>> {
    let podcast_list = block_on(podcast_list(&database_client)).expect("Fail to get the list of podcast from the database");
    Json(podcast_list)
}

#[get("/podcasts/<id>")]
pub fn get_podcast(database_client: State<DatabaseClient>, id: String) -> Json<Podcast> {
    let podcast = block_on(find_podcast_by_id(&database_client, &id)).expect("Fail to find the podcast by id");
    Json(podcast)
}

#[post("/podcasts", format = "json", data = "<new_podcast>")]
pub fn create_podcast(key: ApiKey, database_client: State<DatabaseClient>, new_podcast: Json<Podcast>) -> Json<Podcast> {
    let podcast = new_podcast.into_inner();
    let new_podcast = block_on(insert_podcast(&database_client, &podcast)).expect("Fail to find the podcast by id");
    Json(new_podcast)
}

pub fn initialize_routes() -> Vec<Route> {
    routes![podcasts_list, create_podcast, get_podcast]
}
