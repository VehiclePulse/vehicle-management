use crate::config::app_config::AppConfig;
use crate::rabbitmq::service::RabbitmqPublisher;
use crate::telemetry::service::TelemetryService;
use crate::vehicle::service::VehicleService;

use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub vehicle_service: VehicleService,
    pub telemetry_service: TelemetryService,
    pub rabbitmq: Arc<RabbitmqPublisher>
}
