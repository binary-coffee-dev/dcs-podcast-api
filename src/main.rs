#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[path = "libs/database_client.rs"] mod database_client;
#[path = "libs/routes.rs"] mod routes;
#[path = "libs/models.rs"] mod models;

use futures::executor::block_on;

use database_client::DatabaseClient;

fn main() {
    let mut database = DatabaseClient::new();
    block_on(database.connect()).expect("The database connection fails");

    rocket::ignite()
        .manage(database)
        .mount("/", routes::initialize_routes()).launch();
}
