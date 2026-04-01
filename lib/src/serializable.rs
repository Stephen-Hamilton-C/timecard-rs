use std::str::FromStr;

use chrono::{Weekday, WeekdaySet};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone)]
pub struct SerializableWeekdaySet(pub WeekdaySet);

impl Serialize for SerializableWeekdaySet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let weekdays_str = self.0.iter(Weekday::Sun).map(|w| w.to_string()).join(",");
        serializer.serialize_str(&weekdays_str)
    }
}

impl<'de> Deserialize<'de> for SerializableWeekdaySet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let weekdays_str = <&str>::deserialize(deserializer)?;
        let weekday_set = weekdays_str
            .split(",")
            .map(|wk_str| Weekday::from_str(wk_str).map_err(serde::de::Error::custom))
            .collect::<Result<WeekdaySet, _>>()?;
        Ok(SerializableWeekdaySet(weekday_set))
    }
}
