use futures_util::TryStreamExt;
use mongodb::{Collection, bson::doc};
use uuid::Uuid;

use crate::vehicle::model::Vehicle;

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

    pub async fn get_by_id(&self, id: Uuid) -> mongodb::error::Result<Option<Vehicle>> {
        self.collection.find_one(doc! { "_id": id }).await
    }

    pub async fn get_all(&self) -> mongodb::error::Result<Vec<Vehicle>> {
        let mut cursor = self.collection.find(doc! {}).await?;

        let mut vehicles = Vec::new();

        while let Some(vehicle) = cursor.try_next().await? {
            vehicles.push(vehicle);
        }

        Ok(vehicles)
    }

    pub async fn update(&self, id: Uuid, vehicle: Vehicle) -> mongodb::error::Result<bool> {
        let result = self
            .collection
            .replace_one(doc! {"_id": id}, vehicle)
            .await?;

        Ok(result.modified_count == 1)
    }

    pub async fn delete(&self, id: Uuid) -> mongodb::error::Result<bool> {
        let result = self.collection.delete_one(doc! { "_id": id }).await?;

        Ok(result.deleted_count == 1)
    }
}
