use mongodb::{Client, Database};

use crate::config::Config;

pub async fn connect(config: &Config) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(&config.mongodb_uri).await?;

    client
        .database(&config.mongodb_database)
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await?;

    Ok(client.database(&config.mongodb_database))
}
