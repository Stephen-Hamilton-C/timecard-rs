use std::{fs, path::PathBuf};

use timecard::Timecard;


pub trait TimecardFile {
    // TODO: Error type
    fn load(path: &PathBuf) -> Result<Timecard, Box<dyn std::error::Error>>;

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
}

impl TimecardFile for Timecard {
    // TODO: Error type
    fn load(path: &PathBuf) -> Result<Timecard, Box<dyn std::error::Error>> {
        return if fs::exists(path).unwrap_or(false) {
            let timecard_data = fs::read_to_string(path)?;
            let timecard: Timecard = serde_json::from_str(&timecard_data)?;
            timecard.validate()?;
            Ok(timecard)
        } else {
            let timecard = Timecard::new(vec![])?;
            Ok(timecard)
        }
    }

    fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let timecard_data = serde_json::to_string(self)?;
        fs::write(path, timecard_data)?;
        Ok(())
    }
}
