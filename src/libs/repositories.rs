use futures::stream::StreamExt;
use mongodb::{
    error::Error,
    bson::{doc, to_bson, from_bson, Bson, oid::ObjectId},
    Cursor
};

use crate::libs::models::Podcast;
use crate::libs::database_client::{DatabaseClient, DatabaseBase};


pub async fn podcast_list(database_client: &DatabaseClient) -> Result<Vec<Podcast>, Error> {
    let collection = database_client.collection("podcast");

    let mut cursor: Cursor = collection.find(None, None).await?;

    let mut list: Vec<Podcast> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let podcast: Podcast = from_bson(Bson::Document(document)).expect("Error to create Object");
                list.push(podcast);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(list)
}

pub async fn insert_podcast(database_client: &DatabaseClient, podcast: &Podcast) -> Result<Podcast, Error> {
    let collection = database_client.collection("podcast");

    let mut new_doc = to_bson(&podcast).unwrap().as_document().unwrap().clone();
    new_doc.remove("_id");
    let new_podcast_res = collection.insert_one(new_doc, None).await?;

    let id: String = new_podcast_res.inserted_id.as_object_id().unwrap().clone().to_hex();
    let new_podcast = find_podcast_by_id(database_client, &id).await?;

    Ok(new_podcast)
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

    let result = collection.find_one(doc! { "_id": oid.unwrap() }, None).await?;

    let new_podcast: Podcast = from_bson(Bson::Document(result.unwrap())).expect("Error to create Object");

    Ok(new_podcast)
}
