use futures_util::TryStreamExt;
use mongodb::{Collection, bson::doc};

use crate::vehicle::model::{GetActiveSimulation, Vehicle};

#[derive(Clone)]
pub struct VehicleRepository {
    collection: Collection<Vehicle>,
}

impl VehicleRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("vehicles"),
        }
    }

    pub async fn create(&self, vehicle: Vehicle) -> mongodb::error::Result<()> {
        self.collection.insert_one(vehicle).await?;
        Ok(())
    }

    pub async fn get_by_registration_number(
        &self, 
        registration_number: String
    ) -> mongodb::error::Result<Option<Vehicle>> {
        self.collection
        .find_one(doc! { "registration_number": registration_number })
        .await
    }

    pub async fn get_all(&self, params: GetActiveSimulation) -> mongodb::error::Result<Vec<Vehicle>> {

        let mut filter = doc! {};

        if let Some(is_enabled) = params.enabled {
            filter.insert("simulation", is_enabled);
        }

        let mut cursor = self.collection.find(filter).await?;

        let mut vehicles = Vec::new();

        while let Some(vehicle) = cursor.try_next().await? {
            vehicles.push(vehicle);
        }

        Ok(vehicles)
    }

    pub async fn update(&self, registration_number: String, vehicle: Vehicle) -> mongodb::error::Result<bool> {
        let result = self
            .collection
            .replace_one(doc! {"registration_number": registration_number}, vehicle)
            .await?;

        Ok(result.modified_count == 1)
    }

    pub async fn delete(&self, registration_number: String) -> mongodb::error::Result<bool> {
        let result = self.collection.delete_one(doc! { "registration_number": registration_number }).await?;

        Ok(result.deleted_count == 1)
    }
}
