use std::{fs, path::PathBuf};

use anyhow::Context;
use timecard::Timecard;

pub trait Saveable {
    fn save(&self, path: &PathBuf) -> anyhow::Result<()>;
}

pub trait Loadable<T> {
    fn load(path: &PathBuf) -> anyhow::Result<T>;
}

impl Saveable for Timecard {
    fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        let timecard_data = serde_json::to_string(self)?;
        fs::write(path, timecard_data)?;
        Ok(())
    }
}

impl Loadable<Timecard> for Timecard {
    fn load(path: &PathBuf) -> anyhow::Result<Timecard> {
        return if fs::exists(path).unwrap_or(false) {
            let timecard_data = fs::read_to_string(path)?;
            let timecard: Timecard =
                serde_json::from_str(&timecard_data).context("Failed to parse Timecard")?;
            timecard
                .validate()
                .context("Failed to validate Timecard. Did you manually modify it?")?;
            Ok(timecard)
        } else {
            let timecard = Timecard::new(vec![])?;
            Ok(timecard)
        };
    }
}
