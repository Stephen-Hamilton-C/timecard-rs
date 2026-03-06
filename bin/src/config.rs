use std::{fs, sync::OnceLock};

use chrono::Duration;
use serde::Deserialize;

use crate::traits::Loadable;


const DEFAULT_CONFIG: &str = include_str!("../assets/config.toml");
static INSTANCE: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_use_24_hours")]
    pub use_24_hours: bool,

    #[serde(default = "default_duration_format")]
    pub duration_format: String,

    #[serde(default = "default_work_duration", with = "crate::chrono_humantime")]
    pub work_duration: Duration,

    #[serde(default = "default_entry_lifetime_days")]
    pub entry_lifetime_days: usize,
}

impl Config {
    pub fn get() -> &'static Config {
        INSTANCE.get().unwrap()
    }
}

fn default_use_24_hours() -> bool {
    false
}

fn default_duration_format() -> String {
    String::from("%HH hours, %MM minutes")
}

fn default_work_duration() -> Duration {
    Duration::hours(8)
}

fn default_entry_lifetime_days() -> usize {
    30
}

impl Loadable<&'static Config> for Config {
    fn load(path: &std::path::PathBuf) -> Result<&'static Config, Box<dyn std::error::Error>> {
        if let Some(config) = INSTANCE.get() {
            return Ok(config)
        }

        if fs::exists(path).unwrap_or(false) {
            let config_data = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&config_data)?;
            INSTANCE.set(config).expect("Config already loaded into memory!");
            Ok(Config::get())
        } else {
            fs::write(path, DEFAULT_CONFIG)?;
            Config::load(path)
        }
    }
}
