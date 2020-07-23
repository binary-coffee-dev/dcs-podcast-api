#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod libs;

// use async_std::task::block_on;
use futures::executor::block_on;
use libs::{database_client::{DatabaseClient, DatabaseBase}, routes};

fn main() {
    let mut database = DatabaseClient::new();
    block_on(database.connect()).expect("The database connection fails");

    rocket::ignite()
        .manage(database)
        .mount("/", routes::initialize_routes()).launch();
}
