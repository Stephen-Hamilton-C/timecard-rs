//! Start with `Timecard::new(Vec<TimeEntry>)`, the rest of the functions will explain themselves.
use std::{fmt, str::FromStr};

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{ClockError, TimecardFromStrError, UndoError, ValidationError};

#[cfg(test)]
mod tests;

/// Container for a start and end time
///
/// # Fields
///
/// - `start` (`DateTime<Utc>`) - The time this entry starts.
/// - `end` (`Option<DateTime<Utc>>`) - The time this entry ends.
///                                       If None, this time is assumed to be the current time
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TimeEntry {
    start: DateTime<Utc>,
    /// If None, this time is assumed to be the current time
    end: Option<DateTime<Utc>>,
}

impl TimeEntry {
    /// Create a new TimeEntry and validate it
    ///
    /// # Returns
    ///
    /// - `Result<TimeEntry, ValidationError>` - A validated TimeEntry if OK, otherwise a `ValidationError`
    ///
    /// # Errors
    ///
    /// `error::ValidationError::InvertedEntry`:
    /// - If the end time is before the start time
    pub fn new(
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
    ) -> Result<TimeEntry, ValidationError> {
        let entry = TimeEntry { start, end };

        entry.validate().and(Ok(entry))
    }

    /// Getter for start
    pub fn start(&self) -> DateTime<Utc> {
        self.start
    }

    /// Getter for end
    pub fn end(&self) -> Option<DateTime<Utc>> {
        self.end
    }

    /// Run validation checks on this TimeEntry
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(end) = self.end {
            if self.start > end {
                return Err(ValidationError::InvertedTime);
            }
        }

        Ok(())
    }
}

impl fmt::Display for TimeEntry {
    /// Convert TimeEntry to a serialized string.
    /// Note that TimeEntry derives serde::Serialize,
    /// so you should probably use a serde serializer instead.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self.end {
            Some(end) => write!(f, "{},{}", self.start.to_rfc3339(), end.to_rfc3339()),
            None => write!(f, "{}", self.start.to_rfc3339()),
        };
    }
}

impl FromStr for TimeEntry {
    type Err = chrono::ParseError;

    /// Convert TimeEntry from a serialized string.
    /// Note that TimeEntry derives serde::Deserialize,
    /// so you should probably use a serde deserializer instead.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_split: Vec<&str> = s.split(",").collect();

        let start = DateTime::parse_from_rfc3339(data_split[0])?.with_timezone(&Utc);

        let end: Option<DateTime<Utc>>;
        if data_split.len() > 1 {
            end = Some(DateTime::parse_from_rfc3339(data_split[1])?.with_timezone(&Utc));
        } else {
            end = None;
        }

        Ok(TimeEntry { start, end })
    }
}

/// Manages TimeEntry objects and provides utility functions for adding and removing
/// entries, as well as running calculations on the logged time.
#[derive(Debug, Serialize, Deserialize)]
pub struct Timecard {
    entries: Vec<TimeEntry>,

    /// Purely for testing purposes.
    /// If set, this determines the value for the current time.
    #[serde(skip_serializing)]
    now_override: Option<DateTime<Utc>>,
}

impl Timecard {
    /// Create a new Timecard and validate that TimeEntry objects are in order.
    ///
    /// # Arguments
    ///
    /// - `entries` (`Vec<TimeEntry>`) - The entries to initialize this Timecard with
    ///
    /// # Returns
    ///
    /// - `Result<Timecard, ValidationError>` - A validated Timecard if OK, otherwise a `ValidationError`
    ///
    /// # Errors
    ///
    /// `error::ValidationError::EndTimeRequired`
    /// - A TimeEntry had a None end time, but was not at the end of the entries list
    /// `error::ValidationError::Chronological`
    /// - A TimeEntry's times came before a previous TimeEntry
    /// `error::ValidationError::InvertedEntry`
    /// - A TimeEntry's end time came before its start time
    pub fn new(entries: Vec<TimeEntry>) -> Result<Timecard, ValidationError> {
        let timecard = Timecard {
            entries,
            now_override: None,
        };
        timecard.validate()?;
        Ok(timecard)
    }

    /// Run validation checks on this Timecard
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut prev_time = DateTime::UNIX_EPOCH;
        let entry_count = self.entries.len();
        for (i, entry) in self.entries.iter().enumerate() {
            let is_last_entry = i == entry_count - 1;
            if entry.end.is_none() && !is_last_entry {
                return Err(ValidationError::EndTimeRequired);
            }

            if entry.start < prev_time {
                return Err(ValidationError::Chronological);
            }

            entry.validate()?;
            if let Some(entry_end) = entry.end {
                // If there's no end time, there won't be any more entries,
                // so we don't need to worry about updating prev_time
                prev_time = entry_end;
            }
        }

        Ok(())
    }

    fn now(&self) -> DateTime<Utc> {
        self.now_override.unwrap_or(Utc::now())
    }

    /// Get a read-only slice of all the TimeEntry objects
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

    /// Get a list of entries that overlap the day specified by the given DateTime
    ///
    /// # Arguments
    ///
    /// - `date` (`&DateTime<Utc>`) - The date to filter by
    ///
    /// # Returns
    ///
    /// - `Vec<&TimeEntry>` - A list of entries filtered by the given date
    pub fn filter_by_day(&self, date: &NaiveDate) -> Vec<&TimeEntry> {
        self.filter_by_date_range(date, date)
    }

    /// Get a list of entries that overlap the date range specified by the given DateTimes
    ///
    /// # Arguments
    ///
    /// - `from_date` (`&DateTime<Utc>`) - The date at the start of the filter range, inclusive.
    /// - `to_date` (`&DateTime<Utc>`) - The date at the end of the filter range, inclusive.
    ///
    /// # Returns
    ///
    /// - `Vec<&TimeEntry>` - A list of entries filtered by the given date range
    pub fn filter_by_date_range(
        &self,
        from_date: &NaiveDate,
        to_date: &NaiveDate,
    ) -> Vec<&TimeEntry> {
        let from_day = from_date.num_days_from_ce();
        let to_day = to_date.num_days_from_ce();

        self.entries
            .iter()
            .filter(|&entry| {
                let start_day = entry.start.num_days_from_ce();
                start_day <= to_day
                    && entry
                        .end
                        .map_or(true, |end| end.num_days_from_ce() >= from_day)
            })
            .collect()
    }

    /// Remove all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Start a TimeEntry at the given time
    ///
    /// # Arguments
    ///
    /// - `time` (`DateTime<Utc>`) - The time to start the entry
    ///
    /// # Errors
    ///
    /// `error::ClockError::AlreadyInState(error::ClockState::In)`
    /// - If the Timecard was already clocked in
    /// `error::ClockError::TimeInFuture`
    /// - If the provided time is in the future
    /// `error::ClockError::BeforeLastEntry`
    /// - If the provided time is before the last recorded time
    pub fn clock_in(&mut self, time: DateTime<Utc>) -> Result<(), ClockError> {
        if self.is_clocked_in() {
            return Err(ClockError::AlreadyInState(crate::error::ClockState::In));
        }

        let now = self.now();
        if time > now {
            return Err(ClockError::TimeInFuture(time))
        }

        if let Some(last_entry) = self.entries.last() {
            // If we're clocked out, it's impossible for the last end entry to be None
            if time < last_entry.end.unwrap() {
                return Err(ClockError::BeforeLastEntry(time))
            }
        }

        let entry = TimeEntry {
            start: time,
            end: None,
        };
        self.entries.push(entry);

        Ok(())
    }

    /// End a TimeEntry at the given time
    ///
    /// # Arguments
    ///
    /// - `time` (`DateTime<Utc>`) - The time to end the entry
    ///
    /// # Errors
    ///
    /// `error::ClockError::AlreadyInState(error::ClockState::Out)`
    /// - If the Timecard was already clocked out
    /// `error::ClockError::TimeInFuture`
    /// - If the provided time is in the future
    /// `error::ClockError::BeforeLastEntry`
    /// - If the provided time is before the last recorded time
    pub fn clock_out(&mut self, time: DateTime<Utc>) -> Result<(), ClockError> {
        if self.is_clocked_out() {
            return Err(ClockError::AlreadyInState(crate::error::ClockState::Out));
        }

        let now = self.now();
        if time > now {
            return Err(ClockError::TimeInFuture(time))
        }

        // We're clocked in, so there must be an entry, and the last end entry is None
        let last_entry = self.entries.last_mut().unwrap();
        if time < last_entry.start {
            return Err(ClockError::BeforeLastEntry(time))
        }
        last_entry.end = Some(time);

        Ok(())
    }

    /// Remove the last time from the last TimeEntry
    ///
    /// # Errors
    ///
    /// `error::UndoError::EmptyEntries`
    /// - If the Timecard's entries are empty
    pub fn undo(&mut self) -> Result<(), UndoError> {
        let last_entry = self.entries.last_mut().ok_or(UndoError::EmptyEntries)?;
        if last_entry.end.is_none() {
            self.entries.pop();
        } else {
            last_entry.end = None;
        }

        Ok(())
    }

    /// Get the amount of time clocked in
    ///
    /// # Arguments
    ///
    /// - `date` (`&DateTime<Utc>`) - The date to calculate time worked
    /// - `include_now` (`bool`) - Whether the current time is included in this calculation or not
    ///
    /// # Returns
    ///
    /// - `Duration` - The amount of time clocked in on the date provided.
    pub fn get_duration_worked(&self, date: &NaiveDate, include_now: bool) -> Duration {
        let entries = self.filter_by_day(date);

        let mut total_duration = Duration::zero();
        for entry in entries {
            let Some(end_time) = self.get_last_time_or_now(entry.end, entry.start, include_now)
            else {
                continue;
            };
            total_duration += end_time - entry.start;
        }

        total_duration
    }

    /// Get the amount of time clocked out
    ///
    /// # Arguments
    ///
    /// - `date` (`&DateTime<Utc>`) - The date to calculate time on break.
    /// - `include_now` (`bool`) - Whether the current time is included in this calculation or not
    ///
    /// # Returns
    ///
    /// - `Duration` - The amount of time clocked out on the date provided.
    pub fn get_duration_on_break(&self, date: &NaiveDate, include_now: bool) -> Duration {
        let entries = self.filter_by_day(date);

        let mut total_duration = Duration::zero();
        for (i, &current_entry) in entries.iter().enumerate() {
            let next_entry = entries.get(i + 1);

            if let Some(current_end) = current_entry.end {
                let next_entry_start = next_entry.map(|e| e.start);
                let Some(next_start_time) =
                    self.get_last_time_or_now(next_entry_start, current_end, include_now)
                else {
                    continue;
                };
                total_duration += next_start_time - current_end
            }
        }

        total_duration
    }

    /// Get the time expected to be done working.
    ///
    /// # Arguments
    ///
    /// - `duration_to_work` (`Duration`) - Duration of a work day.
    /// - `date` (`&DateTime<Utc>`) - The date to calculate expected end time.
    ///
    /// # Returns
    ///
    /// - `Option<DateTime<Utc>>` - The date and time expected to be done on the date provided,
    ///                               or None if no entries are found on the given date.
    pub fn get_expected_end_time(
        &self,
        duration_to_work: Duration,
        date: &NaiveDate,
    ) -> Option<DateTime<Utc>> {
        let time_on_break = self.get_duration_on_break(date, true);

        let entries = self.filter_by_day(date);
        let now = self.now();
        if entries.is_empty() && date.num_days_from_ce() != now.num_days_from_ce() {
            return None;
        }

        let start_time = entries.first().map(|e| e.start).unwrap_or(now);
        Some(start_time + duration_to_work + time_on_break)
    }

    fn get_last_time_or_now(
        &self,
        time: Option<DateTime<Utc>>,
        previous_time: DateTime<Utc>,
        include_now: bool,
    ) -> Option<DateTime<Utc>> {
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
    /// Convert Timecard to a serialized string.
    /// Note that Timecard derives serde::Serialize,
    /// so you should probably use a serde serializer instead.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in &self.entries {
            write!(f, "{}\n", entry.to_string())?;
        }

        Ok(())
    }
}

impl FromStr for Timecard {
    type Err = TimecardFromStrError;

    /// Convert Timecard from a serialized string.
    /// Note that Timecard derives serde::Deserialize,
    /// so you should probably use a serde deserializer instead.
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

        Ok(Timecard::new(new_entries)?)
    }
}
