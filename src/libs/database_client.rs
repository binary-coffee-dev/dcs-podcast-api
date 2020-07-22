use mongodb::{Client, options::ClientOptions, error::Error};

pub struct DatabaseClient {
    client: Option<Client>
}

impl DatabaseClient {
    pub fn new() -> DatabaseClient {
        DatabaseClient {
            client: None,
        }
    }
    pub async fn connect(&mut self) -> Result<(), Error> {
        println!("---------Pase----------");
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://localhost:27018").await?;

        // Manually set an option.
        client_options.app_name = Some("My App".to_string());

        // Get a handle to the deployment.
        self.client = Some(Client::with_options(client_options)?);

        // List the names of the databases in that deployment.
        for db_name in self.client.as_ref().unwrap().list_database_names(None, None).await? {
            println!("{}", db_name);
        }
        Ok(())
    }
}
