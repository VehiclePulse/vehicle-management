// use crate::{
//     telemetry::model::{Telemetry, TirePressure},
//     vehicle::model::Vehicle,
//     vehicle::repository::VehicleRepository
// };
// use tokio::time::{interval, Duration};

// pub fn simulate(vehicle: &Vehicle, previous: &Telemetry, elapsed_seconds: f64) -> Telemetry {
//     if !vehicle.simulation.enabled {
//         return previous.clone()
//     }

//     let mut speed = previous.speed;
//     let mut heading = previous.heading;

//     // Gradually increase speed.
//     speed = (speed + 1.0).min(100.0);

//     // Gradually change heading.
//     heading += 1.0;

//     if heading >= 360.0 {
//         heading -= 360.0;
//     }

//     // Speed is represented as km/h.
//     // Convert it to km/s.
//     let distance_km = speed * elapsed_seconds / 3600.0;

//     // Approximate movement on the Earth's surface.
//     let earth_radius_km = 6371.0;
//     let heading_radians = heading.to_radians();

//     let latitude_radians = previous.latitude.to_radians();

//     let delta_latitude =
//         distance_km * heading_radians.cos() / earth_radius_km;

//     let delta_longitude =
//         distance_km * heading_radians.sin()
//             / (earth_radius_km * latitude_radians.cos());

//     let latitude =
//         previous.latitude + delta_latitude.to_degrees();

//     let longitude =
//         previous.longitude + delta_longitude.to_degrees();

//     let fuel_level = (previous.fuel_level - 0.01).max(0.0);

//     let engine_temperature =
//         previous.engine_temperature
//             + if speed > 60.0 { 0.2 } else { -0.1 };

//     let battery_level = (previous.battery_level - 0.005).max(0.0);

//     Telemetry {
//         vehicle_id: previous.vehicle_id,

//         latitude,
//         longitude,

//         speed,
//         heading,

//         fuel_level,
//         engine_temperature,
//         battery_level,

//         tire_pressure: TirePressure {
//             front_left: previous.tire_pressure.front_left,
//             front_right: previous.tire_pressure.front_right,
//             rear_left: previous.tire_pressure.rear_left,
//             rear_right: previous.tire_pressure.rear_right,
//         },

//         last_updated_at: chrono::Utc::now(),
//     }
// }

// pub async fn run(vehicle: Vehicle, initial_telemetry: Telemetry) {
//     let mut telemetry = initial_telemetry;
//     let mut ticker = interval(Duration::from_secs(1));

//     loop {
//         ticker.tick().await;

//         telemetry = simulate(
//             &vehicle,
//             &telemetry,
//             1.0
//         );

//         println!("{:#?}", telemetry)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Utc;
//     use uuid::Uuid;

//     #[test]
//     fn simulation_changes_vehicle_state() {
//         let vehicle_id = Uuid::new_v4();

//         let vehicle = Vehicle {
//             id: vehicle_id,
//             registration_number: "TEST-001".to_string(),
//             make: "Toyota".to_string(),
//             model: "Camry".to_string(),
//             year: 2025,
//             simulation: crate::vehicle::VehicleSimulation {
//                 enabled: true,
//                 initial_latitude: 12.9716,
//                 initial_longitude: 77.5946,
//                 initial_heading: 90.0,
//             },
//         };

//         let telemetry = Telemetry {
//             vehicle_id,
//             latitude: 12.9716,
//             longitude: 77.5946,
//             speed: 50.0,
//             heading: 90.0,
//             fuel_level: 80.0,
//             engine_temperature: 90.0,
//             battery_level: 95.0,
//             tire_pressure: TirePressure {
//                 front_left: 32.0,
//                 front_right: 32.0,
//                 rear_left: 34.0,
//                 rear_right: 34.0,
//             },
//             last_updated_at: Utc::now(),
//         };

//         let next = simulate(&vehicle, &telemetry, 1.0);

//         assert!(next.speed > telemetry.speed);
//         assert!(next.fuel_level < telemetry.fuel_level);
//         assert!(next.battery_level < telemetry.battery_level);
//         assert_ne!(next.heading, telemetry.heading);
//     }
// }

// pub async fn run_simulation(vehicle_repository: VehicleRepository) {
//     let vehicles = match vehicle_repository.get_all().await {
//         Ok(vehicles) => vehicles,
//         Err(error) => {
//             tracing::error!("Failed to load vehicles for simulation: {}", error);
//             return;
//         }
//     };

//     for vehicle in vehicles {
//         if !vehicle.simulation.enabled {
//             continue;
//         }

//         let telemetry = Telemetry {
//             vehicle_id: vehicle.id,
//             latitude: vehicle.simulation.initial_latitude,
//             longitude: vehicle.simulation.initial_longitude,
//             speed: 0.0,
//             heading: vehicle.simulation.initial_heading,
//             fuel_level: 100.0,
//             engine_temperature: 80.0,
//             battery_level: 100.0,
//             tire_pressure: TirePressure {
//                 front_left: 32.0,
//                 front_right: 32.0,
//                 rear_left: 34.0,
//                 rear_right: 34.0,
//             },
//             last_updated_at: chrono::Utc::now(),
//         };

//         tokio::spawn(run(vehicle, telemetry));
//     }
// }
