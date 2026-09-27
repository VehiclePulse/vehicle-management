use crate::config::app_config::AppConfig;
use crate::telemetry::repository::TelemetryRepository;
use crate::vehicle::repository::VehicleRepository;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub vehicle_repository: VehicleRepository,
    pub telemetry_repository: TelemetryRepository,
}
