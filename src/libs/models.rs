use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Podcast {
    pub id: Option<String>,
    pub name: Option<String>,
    pub banner: Option<String>,
    pub url: Option<String>,
}
