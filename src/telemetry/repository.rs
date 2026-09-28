use mongodb::{Collection, bson::doc};
use uuid::Uuid;

use crate::telemetry::model::Telemetry;

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

    pub async fn get_by_vehicle_id(
        &self,
        vehicle_id: Uuid,
    ) -> mongodb::error::Result<Option<Telemetry>> {
        self.collection.find_one(doc! { "_id": vehicle_id }).await
    }

    pub async fn upsert(&self, telemetry: Telemetry) -> mongodb::error::Result<()> {
        let result = self.collection
            .replace_one(doc! { "_id": telemetry.vehicle_id }, telemetry.clone())
            .upsert(true)
            .await?;

        if let Some(_) = &result.upserted_id {
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
