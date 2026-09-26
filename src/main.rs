mod config;
mod database;
mod error;
mod state;
mod telemetry;
mod telemetry_repository;
mod vehicle;
mod vehicle_handler;
mod vehicle_repository;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    let database = database::connect(&config).await?;
    let vehicle_repository = vehicle_repository::VehicleRepository::new(&database);
    let telemetry_repository = telemetry_repository::TelemetryRepository::new(&database);

    let state = AppState {
        database,
        vehicle_repository,
        telemetry_repository,
    };
    let app = axum::Router::new()
        .route(
            "/vehicles",
            axum::routing::get(vehicle_handler::get_vehicles).post(vehicle_handler::create_vehicle),
        )
        .route(
            "/vehicles/{id}",
            axum::routing::get(vehicle_handler::get_vehicle)
                .put(vehicle_handler::update_vehicle)
                .delete(vehicle_handler::delete_vehicle),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    println!("Application started");

    Ok(())
}
