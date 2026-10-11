use mongodb::{Collection, bson::doc};

use crate::{telemetry::model::Telemetry};

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

    // pub async fn add_telemetry(
    //     &self,
    //     telemetry: Telemetry
    // ) -> Result<(), AppError> {
    //     let vehicle = self.collection.find_one(doc! {
    //         "registration_number": telemetry.registration_number.clone()
    //     }).await.map_err(AppError::Database)?; 
        
    //     match vehicle {
    //         Some(_) => {
    //             tracing::debug!(
    //                 registration_number = %telemetry.registration_number,
    //                 "updating telemetry for vehicle"
    //             );

    //             self.collection.insert_one(telemetry)
    //                 .await.map_err(AppError::Database)?;

    //             Ok(())
    //         },
    //         None => {
    //             tracing::error!(
    //                 registration_number = %telemetry.registration_number,
    //                 "no vehicle found to update the telemetry"
    //             );

    //             Err(AppError::NotFound)
    //         }
    //     }

    // }

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
