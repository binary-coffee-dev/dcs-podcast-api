#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod libs;

use rocket_cors;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

// use async_std::task::block_on;
use futures::executor::block_on;
use libs::{database_client::{DatabaseClient, DatabaseBase}, routes};

fn main() -> Result<(), Error> {
    let mut database = DatabaseClient::new();
    block_on(database.connect()).expect("The database connection fails");

    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: false,
        ..Default::default()
    }.to_cors()?;

    rocket::ignite()
        .manage(database)
        .mount("/", routes::initialize_routes())
        .attach(cors)
        .launch();

    Ok(())
}
