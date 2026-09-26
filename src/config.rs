use std::env;

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI must be set"),

            mongodb_database: env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE must be set"),
        }
    }
}
