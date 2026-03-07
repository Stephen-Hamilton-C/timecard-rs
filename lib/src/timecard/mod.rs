use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration, Local};
use serde::{Deserialize, Serialize};

use crate::error::{ClockError, UndoError, ValidationError};

#[cfg(test)]
mod tests;


// TODO: Make properties private and use constructor
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
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Remove unwrap calls
        let data_split: Vec<&str> = s.split(",").collect();

        let start = DateTime::parse_from_rfc3339(data_split[0])?.with_timezone(&Local);

        let end: Option<DateTime<Local>>;
        if data_split.len() > 1 {
            end = Some(DateTime::parse_from_rfc3339(data_split[1])?.with_timezone(&Local));
        } else {
            end = None;
        }

        Ok(TimeEntry { start, end })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Timecard {
    entries: Vec<TimeEntry>,

    #[serde(skip_serializing)]
    now_override: Option<DateTime<Local>>,
}

impl Timecard {
    pub fn new(entries: Vec<TimeEntry>) -> Result<Timecard, ValidationError> {
        let timecard = Timecard { entries, now_override: None };
        timecard.validate()?;
        Ok(timecard)
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut prev_time = DateTime::UNIX_EPOCH.with_timezone(&Local);
        let entry_count = self.entries.len();
        for (i, entry) in self.entries.iter().enumerate() {
            let is_last_entry = i == entry_count - 1;
            if entry.end.is_none() && !is_last_entry {
                return Err(ValidationError::EndTimeRequired)
            }

            if entry.start < prev_time {
                return Err(ValidationError::Chronological)
            }

            if let Some(entry_end) = entry.end {
                if entry.start > entry_end {
                    return Err(ValidationError::InvertedEntry)
                }

                prev_time = entry_end;
            }
        }

        Ok(())
    }

    fn now(&self) -> DateTime<Local> {
        self.now_override.unwrap_or(Local::now())
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

    pub fn filter_by_day(&self, date: &DateTime<Local>) -> Vec<&TimeEntry> {
        self.filter_by_date_range(date, date)
    }

    pub fn filter_by_date_range(&self, from_date: &DateTime<Local>, to_date: &DateTime<Local>) -> Vec<&TimeEntry> {
        let from_day = from_date.num_days_from_ce();
        let to_day = to_date.num_days_from_ce();

        self.entries.iter().filter(|&entry| {
            let start_day = entry.start.num_days_from_ce();
            start_day <= to_day && entry.end.map_or(true, |end| end.num_days_from_ce() >= from_day)
        }).collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn clock_in(&mut self, time: DateTime<Local>) -> Result<(), ClockError> {
        if self.is_clocked_in() {
            return Err(ClockError::AlreadyInState(crate::error::ClockState::In))
        }

        let now = self.now();
        if time > now {
            return Err(ClockError::TimeInFuture)
        }

        if let Some(last_entry) = self.entries.last() {
            // If we're clocked out, it's impossible for the last end entry to be None
            if time < last_entry.end.unwrap() {
                return Err(ClockError::BeforeLastEntry)
            }
        }

        let entry = TimeEntry {
            start: time,
            end: None,
        };
        self.entries.push(entry);

        Ok(())
    }

    pub fn clock_out(&mut self, time: DateTime<Local>) -> Result<(), ClockError> {
        if self.is_clocked_out() {
            return Err(ClockError::AlreadyInState(crate::error::ClockState::Out))
        }

        let now = self.now();
        if time > now {
            return Err(ClockError::TimeInFuture)
        }

        // We're clocked in, so there must be an entry, and the last end entry is None
        let last_entry = self.entries.last_mut().unwrap();
        if time < last_entry.start {
            return Err(ClockError::BeforeLastEntry)
        }
        last_entry.end = Some(time);
        
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), UndoError> {
        let last_entry = self.entries.last_mut().ok_or(UndoError::EmptyEntries)?;
        if last_entry.end.is_none() {
            self.entries.pop();
        } else {
            last_entry.end = None;
        }

        Ok(())
    }

    pub fn get_duration_worked(&self, date: &DateTime<Local>, include_now: bool) -> Duration {
        let entries = self.filter_by_day(date);

        let mut total_duration = Duration::zero();
        for entry in entries {
            let Some(end_time) = self.get_last_time_or_now(entry.end, entry.start, include_now) else { continue };
            total_duration += end_time - entry.start;
        }

        total_duration
    }

    pub fn get_duration_on_break(&self, date: &DateTime<Local>, include_now: bool) -> Duration {
        let entries = self.filter_by_day(date);

        let mut total_duration = Duration::zero();
        for (i, &current_entry) in entries.iter().enumerate() {
            let next_entry = entries.get(i + 1);

            if let Some(current_end) = current_entry.end {
                let next_entry_start = next_entry.map(|e| e.start);
                let Some(next_start_time) = self.get_last_time_or_now(next_entry_start, current_end, include_now) else { continue };
                total_duration += next_start_time - current_end
            }
        }

        total_duration
    }

    pub fn get_expected_end_time(&self, duration_to_work: Duration, date: &DateTime<Local>) -> Option<DateTime<Local>> {
        let time_on_break = self.get_duration_on_break(date, true);

        let entries = self.filter_by_day(date);
        let now = self.now();
        if entries.is_empty() && date.num_days_from_ce() != now.num_days_from_ce() {
            return None
        }

        let start_time = entries.first().map(|e| e.start).unwrap_or(now);
        Some(start_time + duration_to_work + time_on_break)
    }

    fn get_last_time_or_now(&self, time: Option<DateTime<Local>>, previous_time: DateTime<Local>, include_now: bool) -> Option<DateTime<Local>> {
        if time.is_none() {
            let now = self.now();
            if include_now && now.num_days_from_ce() == previous_time.num_days_from_ce() {
                return Some(now);
            }
        }

        time
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
    type Err = chrono::ParseError;
    
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
