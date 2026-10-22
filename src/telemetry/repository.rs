use mongodb::{Collection, bson::doc};
use futures::stream::TryStreamExt;

use crate::{telemetry::model::{Telemetry}};

#[derive(Clone)]
pub struct TelemetryRepository {
    collection: Collection<Telemetry>,
}

impl TelemetryRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("telemetry"),
        }
    }

    pub async fn get_telemetry_for_multiple_vehicles(
        &self,
        registration_numbers: Vec<String>
    ) -> mongodb::error::Result<Vec<Telemetry>>{
        let filter = doc! {
            "registration_number": { "$in": registration_numbers }
        };

        let cursor = self.collection.find(filter)
            .await?;

        let telemetry_list: Vec<Telemetry> = cursor.try_collect().await?;

        Ok(telemetry_list)
    }

    pub async fn get_by_vehicle_id(
        &self,
        registration_number: String,
    ) -> mongodb::error::Result<Option<Telemetry>> {
        self.collection.find_one(doc! { "registration_number": registration_number }).await
    }

    pub async fn upsert(&self, telemetry: Telemetry) -> mongodb::error::Result<()> {
        let result = self.collection
            .replace_one(doc! { "_id": telemetry.vehicle_id }, telemetry.clone())
            .upsert(true)
            .await?;

        if let Some(_) = result.upserted_id {
            tracing::debug!(
                vehicle_id = %telemetry.vehicle_id,
                "document was inserted via upsert"
            );
        } else {
            tracing::debug!(
                vehicle_id = %telemetry.vehicle_id,
                "document was updated via upsert"
            );
        }

        Ok(())
    }
}
