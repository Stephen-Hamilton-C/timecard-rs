use std::{fmt, str::FromStr};

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TimeEntry {
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
}

impl fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.end {
            Some(end) => write!(f, "{},{}", self.start.to_rfc3339(), end.to_rfc3339()),
            None => write!(f, "{}", self.start.to_rfc3339()),
        }
    }
}

impl FromStr for TimeEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Remove unwrap calls
        let data_split: Vec<&str> = s.split(",").collect();

        let start = DateTime::parse_from_rfc3339(data_split[0]).unwrap().with_timezone(&Local);

        let end: Option<DateTime<Local>>;
        if data_split.len() > 1 {
            end = Some(DateTime::parse_from_rfc3339(data_split[1]).unwrap().with_timezone(&Local));
        } else {
            end = None;
        }

        Ok(TimeEntry { start, end })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Timecard {
    entries: Vec<TimeEntry>,
}

impl Timecard {
    pub fn new(entries: Vec<TimeEntry>) -> Result<Timecard, &'static str> {
        let mut prev_time = DateTime::UNIX_EPOCH.with_timezone(&Local);
        let entry_count = entries.len();
        for (i, entry) in entries.iter().enumerate() {
            let is_last_entry = i == entry_count - 1;
            if entry.end.is_none() && !is_last_entry {
                // TODO: Throw error
                return Err("Only the last TimeEntry may have an end time of null!");
            }

            if entry.start < prev_time {
                // TODO: Throw error
                return Err("Timecard must be stored in chronological order!");
            }

            if let Some(entry_end) = entry.end {
                if entry.start > entry_end {
                    // TODO: Throw error
                    return Err("A TimeEntry cannot have a start time that is after an end time!");
                }

                prev_time = entry_end;
            }
        }
        Ok(Timecard { entries })
    }

    pub fn entries(&self) -> &[TimeEntry] {
        &self.entries
    }

    pub fn is_clocked_in(&self) -> bool {
        // Unwrap here should be fine, unless there's a more elegant method
        !self.entries.is_empty() && self.entries.last().unwrap().end.is_none()
    }

    pub fn is_clocked_out(&self) -> bool {
        !self.is_clocked_in()
    }

    pub fn filter_by_day(&self, date: &DateTime<Local>) -> &[TimeEntry] {
        todo!()
    }

    pub fn filter_by_date_range(&self, from_date: &DateTime<Local>, to_date: &DateTime<Local>) -> &[TimeEntry] {
        todo!()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    // TODO: Error should not be string
    pub fn clock_in(&mut self, time: &DateTime<Local>) -> Result<(), String> {
        todo!()
    }

    // TODO: Error should not be string
    pub fn clock_out(&mut self, time: &DateTime<Local>) -> Result<(), String> {
        todo!()
    }

    // TODO: Error should not be string
    pub fn undo(&mut self) -> Result<(), String> {
        todo!()
    }

    pub fn get_duration_worked(&self, date: &DateTime<Local>, include_now: bool) -> Duration {
        todo!()
    }

    pub fn get_duration_on_break(&self, date: &DateTime<Local>, include_now: bool) -> Duration {
        todo!()
    }

    pub fn get_expected_end_time(&self, duration_to_work: &Duration, date: &DateTime<Local>) -> DateTime<Local> {
        todo!()
    }
}

impl fmt::Display for Timecard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries {
            write!(f, "{}\n", entry.to_string())?;
        }

        Ok(())
    }
}

impl FromStr for Timecard {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new_entries = vec![];
        let entries_data: Vec<&str> = s.split("\n").collect();
        for entry_data in entries_data {
            if entry_data.is_empty() {
                continue;
            }

            let entry = TimeEntry::from_str(entry_data)?;
            new_entries.push(entry);
        }

        // TODO: Remove unwrap call
        Ok(Timecard::new(new_entries).unwrap())
    }
}
