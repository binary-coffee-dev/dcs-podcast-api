#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[path = "libs/database_client.rs"] mod database_client;
#[path = "libs/routes.rs"] mod routes;

use futures::executor::block_on;

use database_client::DatabaseClient;

fn main() {
    let mut database = DatabaseClient::new();
    block_on(database.connect()).expect("The database connection fails");

    rocket::ignite().mount("/", routes::initialize_routes()).launch();
}
