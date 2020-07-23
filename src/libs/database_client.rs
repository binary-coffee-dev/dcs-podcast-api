use mongodb::{Client, options::ClientOptions, error::Error, Database};

pub struct DatabaseClient {
    pub db: Option<Database>
}

impl DatabaseClient {
    pub fn new() -> DatabaseClient {
        DatabaseClient {
            db: None
        }
    }
    pub async fn connect(&mut self) -> Result<(), Error> {
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27018").await?;

        // Manually set an option.
        client_options.app_name = Some("Podcast API".to_string());

        // Get a handle to the deployment.
        let client = Some(Client::with_options(client_options)?);

        // Get database instance
        self.db = Some(client.as_ref().unwrap().database("podcast_db"));

        Ok(())
    }
}
