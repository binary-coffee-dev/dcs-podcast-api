use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Deserialize, Debug, Clone)]
pub struct Podcast {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub banner: Option<String>,
    pub url: Option<String>,
    pub duration: Option<f32>,
    pub date: Option<DateTime>
}

impl Serialize for Podcast {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Podcast", 6)?;
        match &self._id {
            Some(id) => {
                state.serialize_field("id", &id.to_string())?
            }
            None => {
                state.serialize_field("id", &self._id)?;
            }
        }
        match &self.date {
            Some(date) => {
                state.serialize_field("date", &date.to_rfc3339())?
            }
            None => {
                state.serialize_field("date", &self.date)?;
            }
        }
        state.serialize_field("name", &self.name)?;
        state.serialize_field("banner", &self.banner)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("duration", &self.duration)?;
        state.end()
    }
}

impl Default for Podcast {
    fn default() -> Self {
        return Podcast {
            _id: None,
            name: None,
            banner: None,
            url: None,
            duration: None,
            date: None
        }
    }
}
