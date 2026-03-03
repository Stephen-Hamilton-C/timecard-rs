use std::{fmt, str::FromStr};

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};


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

    // TODO: Error should not be string
    pub fn clean(&mut self, past_date: Option<&DateTime<Local>>) -> Result<(), String> {
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

    pub fn get_duration_worked(&self, date: &DateTime<Local>, include_now: Option<bool>) -> Duration {
        todo!()
    }

    pub fn get_duration_on_break(&self, date: &DateTime<Local>, include_now: Option<bool>) -> Duration {
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

#[cfg(test)]
mod tests {
    use std::io::empty;

    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn it_rejects_bad_entry() {
        let now = Local::now();
        let entries = vec![
            TimeEntry {
                start: now - Duration::minutes(2),
                end: Some(Local::now() - Duration::minutes(1))
            },
            TimeEntry {
                start: now,
                end: Some(now - Duration::milliseconds(1)),
            },
            TimeEntry {
                start: now + Duration::minutes(1),
                end: None,
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_bad_order() {
        let now = Local::now();
        let entries = vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(5)),
            },
            TimeEntry {
                start: now - Duration::minutes(20),
                end: Some(now - Duration::minutes(15)),
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_bad_none_end_entry_order() {
        let now = Local::now();
        let entries = vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: None,
            },
            TimeEntry {
                start: now - Duration::minutes(5),
                end: None,
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_creates_timecard() -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        Timecard::new(vec![]);
        Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(9)),
            },
            TimeEntry {
                start: now - Duration::minutes(8),
                end: Some(now - Duration::minutes(7)),
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(5)),
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(5)),
            },
            TimeEntry {
                start: now - Duration::minutes(1),
                end: None,
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(1),
                end: None,
            },
        ])?;

        Ok(())
    }

    #[test]
    fn it_gets_entries() -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        let entries = vec![TimeEntry {
                start: now - Duration::minutes(5),
                end: None,
            },
        ];
        let timecard = Timecard::new(entries.clone())?;
        assert_eq!(entries.as_slice(), timecard.entries());

        Ok(())
    }

    #[test]
    fn it_tracks_clocked_state() -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();

        let timecard1 = Timecard::new(vec![])?;
        assert!(!timecard1.is_clocked_in());
        assert!(timecard1.is_clocked_out());

        let timecard2 = Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: None,
            },
        ])?;
        assert!(timecard2.is_clocked_in());
        assert!(!timecard2.is_clocked_out());

        let timecard3 = Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(5)),
            },
            TimeEntry {
                start: now - Duration::minutes(3),
                end: Some(now - Duration::minutes(1)),
            },
        ])?;
        assert!(!timecard3.is_clocked_in());
        assert!(timecard3.is_clocked_out());

        let timecard4 = Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: Some(now - Duration::minutes(5)),
            },
            TimeEntry {
                start: now - Duration::minutes(3),
                end: None,
            },
        ])?;
        assert!(timecard4.is_clocked_in());
        assert!(!timecard4.is_clocked_out());

        Ok(())
    }

    #[test]
    fn it_filters_by_day() -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        let empty_entries = vec![];
        let timecard1 = Timecard::new(empty_entries.clone())?;
        assert_eq!(empty_entries, timecard1.filter_by_day(&now));
        assert_eq!(empty_entries, timecard1.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(empty_entries, timecard1.filter_by_day(&(now - Duration::weeks(52))));

        let entries2 = vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: None,
            },
        ];
        let timecard2 = Timecard::new(entries2.clone())?;
        assert_eq!(entries2, timecard2.filter_by_day(&now));
        assert_eq!(entries2, timecard2.filter_by_day(&(now - Duration::minutes(30))));
        assert_eq!(empty_entries, timecard2.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(empty_entries, timecard2.filter_by_day(&(now - Duration::weeks(52))));

        let entries3 = vec![
            TimeEntry {
                start: now - Duration::days(3) - Duration::minutes(10),
                end: Some(now - Duration::days(3)),
            }
        ];
        let timecard3 = Timecard::new(entries3.clone())?;
        assert_eq!(entries3, timecard3.filter_by_day(&(now - Duration::days(3))));
        assert_eq!(empty_entries, timecard3.filter_by_day(&(now - Duration::days(2))));
        assert_eq!(empty_entries, timecard3.filter_by_day(&(now - Duration::days(4))));
        assert_eq!(empty_entries, timecard3.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(empty_entries, timecard3.filter_by_day(&now));

        let entries4 = vec![
            TimeEntry {
                start: now - Duration::days(3) - Duration::minutes(10),
                end: Some(now - Duration::days(3)),
            },
            TimeEntry {
                start: now - Duration::days(2) - Duration::minutes(10),
                end: Some(now - Duration::days(2)),
            },
            TimeEntry {
                start: now - Duration::days(1) - Duration::minutes(10),
                end: Some(now - Duration::days(1)),
            },
        ];
        let timecard4 = Timecard::new(entries4.clone())?;
        assert_eq!(vec![entries4[0].clone()], timecard4.filter_by_day(&(now - Duration::days(3))));
        assert_eq!(vec![entries4[1].clone()], timecard4.filter_by_day(&(now - Duration::days(2))));
        assert_eq!(vec![entries4[2].clone()], timecard4.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(empty_entries, timecard4.filter_by_day(&(now - Duration::days(4))));
        assert_eq!(empty_entries, timecard4.filter_by_day(&now));

        let entries5 = vec![
            TimeEntry {
                start: now - Duration::days(3) - Duration::minutes(10),
                end: Some(now - Duration::days(3)),
            },
            TimeEntry {
                start: now - Duration::days(2) - Duration::minutes(10),
                end: Some(now - Duration::days(1)),
            },
        ];
        let timecard5 = Timecard::new(entries5.clone())?;
        assert_eq!(vec![entries5[0].clone()], timecard5.filter_by_day(&(now - Duration::days(3))));
        assert_eq!(vec![entries5[1].clone()], timecard5.filter_by_day(&(now - Duration::days(2))));
        assert_eq!(vec![entries5[1].clone()], timecard5.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(empty_entries, timecard5.filter_by_day(&(now - Duration::days(4))));
        assert_eq!(empty_entries, timecard5.filter_by_day(&now));

        let entries6 = vec![
            TimeEntry {
                start: now - Duration::days(3) - Duration::minutes(10),
                end: Some(now - Duration::days(3)),
            },
            TimeEntry {
                start: now - Duration::days(2) - Duration::minutes(10),
                end: None,
            },
        ];
        let timecard6 = Timecard::new(entries6.clone())?;
        assert_eq!(vec![entries6[0].clone()], timecard6.filter_by_day(&(now - Duration::days(3))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&(now - Duration::days(2))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&(now - Duration::days(1))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&now));

        Ok(())
    }

    #[test]
    fn it_clears() -> Result<(), Box<dyn std::error::Error>> {
        let now = Local::now();
        let mut timecard = Timecard::new(vec![
            TimeEntry {
                start: now - Duration::minutes(10),
                end: None,
            },
        ])?;

        assert!(!timecard.entries.is_empty());
        timecard.clear();
        assert!(timecard.entries.is_empty());

        Ok(())
    }
}
