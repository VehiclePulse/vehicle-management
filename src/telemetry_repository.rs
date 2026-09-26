use mongodb::Collection;

use crate::telemetry::Telemetry;

#[derive(Clone)]
pub struct TelemetryRepository {
    collection: Collection<Telemetry>,
}

impl TelemetryRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        Self {
            collection: database.collection("vehicle_telemetry"),
        }
    }
}
