use async_trait::async_trait;
use mongodb::{Client, options::ClientOptions, error::Error, Database, Collection};

use crate::libs::utils::get_env;

#[async_trait]
pub trait DatabaseBase {
    fn new() -> DatabaseClient;
    async fn connect(&mut self) -> Result<(), Error>;

    fn collection(&self, collection_name: &str) -> Collection;
}

pub struct DatabaseClient {
    pub db: Option<Database>
}

#[async_trait]
impl DatabaseBase for DatabaseClient {
    fn new() -> DatabaseClient {
        DatabaseClient {
            db: None
        }
    }
    async fn connect(&mut self) -> Result<(), Error> {
        let host = get_env("DB_HOST", "localhost");
        let port = get_env("DB_PORT", "27018");
        let db_name = get_env("DB_NAME", "podcast_db");

        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse(format!("mongodb://{}:{}", host.as_str(), port.as_str()).as_str()).await?;

        // Manually set an option.
        client_options.app_name = Some("Podcast API".to_string());

        // Get a handle to the deployment.
        let client = Some(Client::with_options(client_options)?);

        // Get database instance
        self.db = Some(client.as_ref().unwrap().database(db_name.as_str()));

        Ok(())
    }

    fn collection(&self, collection_name: &str) -> Collection {
        self.db.as_ref().unwrap().collection(collection_name)
    }
}
