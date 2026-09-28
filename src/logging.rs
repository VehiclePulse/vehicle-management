use tracing_subscriber::filter::LevelFilter;

pub fn init(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "none" => LevelFilter::OFF,
        value => value
            .parse::<LevelFilter>()
            .unwrap_or(LevelFilter::INFO)
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
}