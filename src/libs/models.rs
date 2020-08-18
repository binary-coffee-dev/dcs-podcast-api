use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Podcast {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub banner: Option<String>,
    pub url: Option<String>,
}

impl Default for Podcast {
    fn default() -> Self {
        return Podcast {
            _id: None,
            name: None,
            banner: None,
            url: None
        }
    }
}
