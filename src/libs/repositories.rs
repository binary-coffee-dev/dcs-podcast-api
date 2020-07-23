use mongodb::error::Error;
use mongodb::{
    bson::{doc, to_bson, Bson},
    options::{FindOptions, FindOneOptions},
};

use crate::models::Podcast;
use crate::database_client::DatabaseClient;

pub async fn insert_podcast(database_client: &DatabaseClient, item: &Podcast) -> Result<(), Error> {
    let collection = database_client.db.as_ref().unwrap().collection("podcast");

    let item_bson = to_bson(&item).unwrap();
    collection.insert_one(item_bson.as_document().unwrap().clone(), None).await?;

    Ok(())
}

pub async fn podcast_list(database_client: &DatabaseClient) -> Result<Vec<Podcast>, Error> {
    let collection = database_client.db.as_ref().unwrap().collection("podcast");

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
    let collection = database_client.db.as_ref().unwrap().collection("podcast");

    let filter = doc! { "_id": "5f18b1f70091c38500eb0b97" };
    let find_options = FindOneOptions::builder().build();
    let result = collection.find_one(filter, find_options).await?;

    println!("this {}", &id);
    println!("this {}", result.unwrap_or(doc! {}));

    Ok(Podcast {
        id: Some(String::from("asdf")),
        name: Some(String::from("asdf")),
        banner: Some(String::from("asdf")),
        url: Some(String::from("asdf"))
    })
}
