use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Podcast {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub banner: Option<String>,
    pub url: Option<String>,
}
