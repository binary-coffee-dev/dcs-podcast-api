extern crate mongodb;

use mongodb::error::Error;
use mongodb::{
    bson::{doc, to_bson, from_bson, Bson, bson, oid::ObjectId},
    options::{FindOptions, FindOneOptions},
};

use crate::models::Podcast;
use crate::database_client::{DatabaseClient, DatabaseBase};

pub async fn insert_podcast(database_client: &DatabaseClient, podcast: &Podcast) -> Result<Podcast, Error> {
    let collection = database_client.collection("podcast");

    let item_bson = to_bson(&podcast).unwrap();
    let new_podcast_res = collection.insert_one(item_bson.as_document().unwrap().clone(), None).await?;

    println!("aaaa {:#?}", new_podcast_res);

    let id = new_podcast_res.inserted_id;
    // match new_podcast_res {
        // Ok(podcast) => {
            // let new_podcast: Podcast = from_bson(podcast.inserted_id);
            // Ok(new_podcast)
        // }
    // }

    Ok(podcast.clone())
}

pub async fn podcast_list(database_client: &DatabaseClient) -> Result<Vec<Podcast>, Error> {
    let collection = database_client.collection("podcast");

    let filter = doc! { };
    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await?;

    let mut list: Vec<Podcast> = Vec::new();

    // while let Some(result) = cursor.next().await {
    //     match result {
    //         Ok(document) => {
    //             if let Some(title) = document.get("title").and_then(Bson::as_str) {
    //                 println!("title: {}", title);
    //             }  else {
    //                 println!("no title found");
    //             }
    //         }
    //         Err(e) => return Err(e.into()),
    //     }
    // }

    Ok(list)
}

pub async fn find_podcast_by_id(database_client: &DatabaseClient, id: &String) -> Result<Podcast, Error> {
    let collection = database_client.collection("podcast");

    let mut oid = to_bson(id);
    match ObjectId::with_string(id) {
        Ok(o_id) => {
            oid = to_bson(&o_id);
        }
        Err(e) => {
            println!("Error with the oid {:#?}", e);
        }
    }

    let result = collection.find_one(doc! { "_id": oid.unwrap() }, FindOneOptions::builder().build()).await?;

    let new_podcast: Podcast = from_bson(bson!{result.as_ref().unwrap()}).expect("Error to create Object");

    Ok(new_podcast)
}
