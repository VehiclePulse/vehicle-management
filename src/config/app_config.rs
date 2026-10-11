use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub database_config: DatabaseConfig,
    pub server: Server,
    pub log_level: String
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

#[derive(Deserialize, Clone)]
pub struct Server {
    pub url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load_config() -> Self {
        let env = std::env::var("Environment").unwrap_or_else(|_| "Developement".to_string());

        let config_path = match env.as_str() {
            "Production" => Path::new("/app/config/config.json").to_path_buf(),
            _ => Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("config.json"),
        };

        tracing::debug!(
            mode = %env,
            "Starting application"
        );

        let file = File::open(&config_path).unwrap_or_else(|err| {
            panic!(
                "config file not found in the path: {}, error: {}",
                config_path.display(),
                err
            )
        });

        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
            .unwrap_or_else(|err| panic!("error occurred while reading json file: {}", err))
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn loads_config_from_default_environment() {
//         // std::env::remove_var("Environment");

//         let config = AppConfig::load_config();

//         assert!(!config.database_config.mongodb_database.is_empty());
//         assert!(!config.database_config.mongodb_uri.is_empty());
//         assert!(!config.server.url.is_empty());
//         assert!(!config.server.port > 0);
//         assert!(!config.log_level.is_empty());
//     }   
// }