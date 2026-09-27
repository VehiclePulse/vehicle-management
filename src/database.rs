use mongodb::{Client, Database};

use crate::config::app_config::DatabaseConfig;

pub async fn connect(db_config: &DatabaseConfig) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(&db_config.mongodb_uri).await?;

    client
        .database(&db_config.mongodb_database)
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await?;

    Ok(client.database(&db_config.mongodb_database))
}
