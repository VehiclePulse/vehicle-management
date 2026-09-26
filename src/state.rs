use mongodb::Database;

use crate::telemetry_repository::TelemetryRepository;
use crate::vehicle_repository::VehicleRepository;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub vehicle_repository: VehicleRepository,
    pub telemetry_repository: TelemetryRepository,
}
