use chrono::Duration;
use serde::{Deserialize, Deserializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let std_duration = humantime::parse_duration(&s).map_err(serde::de::Error::custom)?;
    Duration::from_std(std_duration).map_err(serde::de::Error::custom)
}
