use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Debug, Clone)]
pub struct Podcast {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub banner: Option<String>,
    pub url: Option<String>,
}

impl Serialize for Podcast {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Podcast", 4)?;
        match &self._id {
            Some(id) => {
                state.serialize_field("id", &id.to_string())?
            }
            None => {
                state.serialize_field("id", &self._id)?;
            }
        }
        state.serialize_field("name", &self.name)?;
        state.serialize_field("banner", &self.banner)?;
        state.serialize_field("url", &self.url)?;
        state.end()
    }
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
