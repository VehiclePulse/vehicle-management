use std::sync::Arc;

use vehicle_management::{
    database,
    config::app_config::AppConfig,
    vehicle,
    telemetry,
    state::AppState,
    routes,
    middleware,
    logging,
    rabbitmq
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let config = AppConfig::load_config();

    let database = database::connect(&config.database_config).await?;
    let vehicle_repository = vehicle::repository::VehicleRepository::new(&database);
    let telemetry_repository = telemetry::repository::TelemetryRepository::new(&database);
    let vehicle_service = vehicle::service::VehicleService::new(vehicle_repository);
    let telemetry_service = telemetry::service::TelemetryService::new(telemetry_repository);
    let rabbitmq_service = rabbitmq::service::RabbitmqPublisher::new(
        config.rabbitmq.url.clone(), config.rabbitmq.exchange.clone()).await?;

    let state = AppState {
        config,
        vehicle_service,
        telemetry_service,
        rabbitmq: Arc::new(rabbitmq_service)
    };

    let server_url = format!("{}:{}", state.config.server.url, state.config.server.port);
    logging::init(&state.config.log_level);


    let app = routes::create_router()
        .with_state(state)
        .layer(axum::middleware::from_fn(middleware::logging::request_logger));

    tracing::info!(
        server_url = %server_url,
        "Starting server"
    );

    let listener = tokio::net::TcpListener::bind(server_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
