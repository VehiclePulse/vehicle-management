mod config;
mod database;
mod error;
mod routes;
mod simulator;
mod state;
mod telemetry;
mod vehicle;
mod extractors;
mod middleware;
mod logging;

use config::app_config::AppConfig;
use state::AppState;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = AppConfig::load_config();

    let database = database::connect(&config.database_config).await?;
    let vehicle_repository = vehicle::repository::VehicleRepository::new(&database);
    let telemetry_repository = telemetry::repository::TelemetryRepository::new(&database);

    let state = AppState {
        config,
        vehicle_repository,
        telemetry_repository,
    };

    let server_url = format!("{}:{}", state.config.server.url, state.config.server.port);
    logging::init(&state.config.log_level);


    let app = routes::create_router()
        .with_state(state)
        .layer(axum::middleware::from_fn(crate::middleware::logging::request_logger));

    tracing::info!(
        server_url = %server_url,
        "Starting server"
    );

    let listener = tokio::net::TcpListener::bind(server_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
