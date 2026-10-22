use crate::config::app_config::AppConfig;
use crate::telemetry::service::TelemetryService;
use crate::vehicle::service::VehicleService;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub vehicle_service: VehicleService,
    pub telemetry_service: TelemetryService,
}
