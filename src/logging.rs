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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::process::Command;

//     #[test]
//     fn init_with_info_level() {
//         let executable = std::env::current_exe().unwrap();

//         let output = Command::new(executable)
//             .arg("child_init_info")
//             .output()
//             .unwrap();

//         assert!(output.status.success());
//     }

//     #[test]
//     fn child_init_info() {
//         init("info");
//     }
// }