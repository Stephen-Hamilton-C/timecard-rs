use std::path::PathBuf;

use chrono::Utc;

pub mod core;

pub const NAME: &str = "timecard";
pub const AUTHOR: &str = "Stephen-Hamilton-C";

pub fn data_dir() -> PathBuf {
    appdirs::user_data_dir(Some(NAME), Some(AUTHOR), false)
        .expect("Unknown error while retrieving data directory.")
}

pub fn config_dir() -> PathBuf {
    appdirs::user_config_dir(Some(NAME), Some(AUTHOR), false)
        .expect("Unknown error while retrieving config directory.")
}

pub fn timecard_path() -> PathBuf {
    data_dir().join(format!("timecard_{}.json", Utc::now().date_naive().to_string()))
}
