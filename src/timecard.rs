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
pub mod tests {
    use super::*;

    fn get_ref_time() -> DateTime<Local> {
        Local::now()
    }

    #[test]
    fn it_rejects_bad_entry() {
        let time = get_ref_time();
        let entries = vec![
            TimeEntry {
                start: time - Duration::minutes(2),
                end: Some(time - Duration::minutes(1))
            },
            TimeEntry {
                start: time,
                end: Some(time - Duration::milliseconds(1)),
            },
            TimeEntry {
                start: time + Duration::minutes(1),
                end: None,
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_bad_order() {
        let time = get_ref_time();
        let entries = vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(5)),
            },
            TimeEntry {
                start: time - Duration::minutes(20),
                end: Some(time - Duration::minutes(15)),
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_rejects_bad_none_end_entry_order() {
        let time = get_ref_time();
        let entries = vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: None,
            },
            TimeEntry {
                start: time - Duration::minutes(5),
                end: None,
            },
        ];
        let result = Timecard::new(entries);
        // TODO: Assert specific error
        assert!(result.is_err());
    }

    #[test]
    fn it_creates_timecard() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        Timecard::new(vec![]);
        Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(9)),
            },
            TimeEntry {
                start: time - Duration::minutes(8),
                end: Some(time - Duration::minutes(7)),
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(5)),
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(5)),
            },
            TimeEntry {
                start: time - Duration::minutes(1),
                end: None,
            },
        ])?;
        Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(1),
                end: None,
            },
        ])?;

        Ok(())
    }

    #[test]
    fn it_gets_entries() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        let entries = vec![TimeEntry {
                start: time - Duration::minutes(5),
                end: None,
            },
        ];
        let timecard = Timecard::new(entries.clone())?;
        assert_eq!(entries.as_slice(), timecard.entries());

        Ok(())
    }

    #[test]
    fn it_tracks_clocked_state() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();

        let timecard1 = Timecard::new(vec![])?;
        assert!(!timecard1.is_clocked_in());
        assert!(timecard1.is_clocked_out());

        let timecard2 = Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: None,
            },
        ])?;
        assert!(timecard2.is_clocked_in());
        assert!(!timecard2.is_clocked_out());

        let timecard3 = Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(5)),
            },
            TimeEntry {
                start: time - Duration::minutes(3),
                end: Some(time - Duration::minutes(1)),
            },
        ])?;
        assert!(!timecard3.is_clocked_in());
        assert!(timecard3.is_clocked_out());

        let timecard4 = Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: Some(time - Duration::minutes(5)),
            },
            TimeEntry {
                start: time - Duration::minutes(3),
                end: None,
            },
        ])?;
        assert!(timecard4.is_clocked_in());
        assert!(!timecard4.is_clocked_out());

        Ok(())
    }

    #[test]
    fn it_filters_by_day() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        let timecard1 = Timecard::new(vec![])?;
        assert!(timecard1.filter_by_day(&time).is_empty());
        assert!(timecard1.filter_by_day(&(time - Duration::days(1))).is_empty());
        assert!(timecard1.filter_by_day(&(time - Duration::weeks(52))).is_empty());

        let entries2 = vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: None,
            },
        ];
        let timecard2 = Timecard::new(entries2.clone())?;
        assert_eq!(entries2, timecard2.filter_by_day(&time));
        assert_eq!(entries2, timecard2.filter_by_day(&(time - Duration::minutes(30))));
        assert!(timecard2.filter_by_day(&(time - Duration::days(1))).is_empty());
        assert!(timecard2.filter_by_day(&(time - Duration::weeks(52))).is_empty());

        let entries3 = vec![
            TimeEntry {
                start: time - Duration::days(3) - Duration::minutes(10),
                end: Some(time - Duration::days(3)),
            }
        ];
        let timecard3 = Timecard::new(entries3.clone())?;
        assert_eq!(entries3, timecard3.filter_by_day(&(time - Duration::days(3))));
        assert!(timecard3.filter_by_day(&(time - Duration::days(2))).is_empty());
        assert!(timecard3.filter_by_day(&(time - Duration::days(4))).is_empty());
        assert!(timecard3.filter_by_day(&(time - Duration::days(1))).is_empty());
        assert!(timecard3.filter_by_day(&time).is_empty());

        let entries4 = vec![
            TimeEntry {
                start: time - Duration::days(3) - Duration::minutes(10),
                end: Some(time - Duration::days(3)),
            },
            TimeEntry {
                start: time - Duration::days(2) - Duration::minutes(10),
                end: Some(time - Duration::days(2)),
            },
            TimeEntry {
                start: time - Duration::days(1) - Duration::minutes(10),
                end: Some(time - Duration::days(1)),
            },
        ];
        let timecard4 = Timecard::new(entries4.clone())?;
        assert_eq!(vec![entries4[0].clone()], timecard4.filter_by_day(&(time - Duration::days(3))));
        assert_eq!(vec![entries4[1].clone()], timecard4.filter_by_day(&(time - Duration::days(2))));
        assert_eq!(vec![entries4[2].clone()], timecard4.filter_by_day(&(time - Duration::days(1))));
        assert!(timecard4.filter_by_day(&(time - Duration::days(4))).is_empty());
        assert!(timecard4.filter_by_day(&time).is_empty());

        let entries5 = vec![
            TimeEntry {
                start: time - Duration::days(3) - Duration::minutes(10),
                end: Some(time - Duration::days(3)),
            },
            TimeEntry {
                start: time - Duration::days(2) - Duration::minutes(10),
                end: Some(time - Duration::days(1)),
            },
        ];
        let timecard5 = Timecard::new(entries5.clone())?;
        assert_eq!(vec![entries5[0].clone()], timecard5.filter_by_day(&(time - Duration::days(3))));
        assert_eq!(vec![entries5[1].clone()], timecard5.filter_by_day(&(time - Duration::days(2))));
        assert_eq!(vec![entries5[1].clone()], timecard5.filter_by_day(&(time - Duration::days(1))));
        assert!(timecard5.filter_by_day(&(time - Duration::days(4))).is_empty());
        assert!(timecard5.filter_by_day(&time).is_empty());

        let entries6 = vec![
            TimeEntry {
                start: time - Duration::days(3) - Duration::minutes(10),
                end: Some(time - Duration::days(3)),
            },
            TimeEntry {
                start: time - Duration::days(2) - Duration::minutes(10),
                end: None,
            },
        ];
        let timecard6 = Timecard::new(entries6.clone())?;
        assert_eq!(vec![entries6[0].clone()], timecard6.filter_by_day(&(time - Duration::days(3))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&(time - Duration::days(2))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&(time - Duration::days(1))));
        assert_eq!(vec![entries6[1].clone()], timecard6.filter_by_day(&time));

        Ok(())
    }

    #[test]
    fn it_filters_by_date_range() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();

        let entries1 = vec![
            TimeEntry {
                start: time - Duration::days(14) - Duration::hours(8),
                end: Some(time - Duration::days(14)),
            },
            TimeEntry {
                start: time - Duration::days(13) - Duration::hours(8),
                end: Some(time - Duration::days(13)),
            },
            TimeEntry {
                start: time - Duration::days(12) - Duration::hours(4),
                end: Some(time - Duration::days(12) - Duration::hours(3) - Duration::minutes(30)),
            },
            TimeEntry {
                start: time - Duration::days(12) - Duration::hours(3),
                end: Some(time - Duration::days(12)),
            },
            TimeEntry {
                start: time - Duration::days(10) - Duration::hours(3),
                end: Some(time - Duration::days(8)),
            },
        ];
        let timecard1 = Timecard::new(entries1.clone())?;
        assert_eq!(vec![entries1[0].clone()], timecard1.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(14))));
        assert_eq!(vec![entries1[1].clone()], timecard1.filter_by_date_range(&(time - Duration::days(13)), &(time - Duration::days(13))));
        assert_eq!(
            vec![
                entries1[0].clone(),
                entries1[1].clone(),
            ],
            timecard1.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(13))),
        );
        assert_eq!(
            vec![
                entries1[0].clone(),
                entries1[1].clone(),
                entries1[2].clone(),
                entries1[3].clone(),
            ],
            timecard1.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(12))),
        );
        assert_eq!(entries1, timecard1.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(8))));
        assert_eq!(entries1, timecard1.filter_by_date_range(&(time - Duration::days(20)), &time));
        assert_eq!(vec![entries1[4].clone()], timecard1.filter_by_date_range(&(time - Duration::days(10)), &(time - Duration::days(8))));
        
        let entries2 = vec![
            TimeEntry {
                start: time - Duration::days(2),
                end: None,
            },
        ];
        let timecard2 = Timecard::new(entries2.clone())?;
        assert_eq!(entries2, timecard2.filter_by_date_range(&(time - Duration::days(2)), &Local::now()));
        assert_eq!(entries2, timecard2.filter_by_date_range(&(time - Duration::days(2)), &(time - Duration::days(2))));
        assert_eq!(entries2, timecard2.filter_by_date_range(&(time - Duration::days(3)), &Local::now()));

        Ok(())
    }

    #[test]
    fn it_clears() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        let mut timecard = Timecard::new(vec![
            TimeEntry {
                start: time - Duration::minutes(10),
                end: None,
            },
        ])?;

        assert!(!timecard.entries.is_empty());
        timecard.clear();
        assert!(timecard.entries.is_empty());

        Ok(())
    }

    #[test]
    fn it_clocks_in() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();

        let mut timecard1 = Timecard::new(vec![])?;
        assert!(timecard1.entries.is_empty());
        assert!(!timecard1.is_clocked_in());
        assert!(timecard1.is_clocked_out());

        timecard1.clock_in(&time)?;

        assert_eq!(
            vec![TimeEntry { start: time, end: None }],
            timecard1.entries,
        );
        assert!(timecard1.is_clocked_in());
        assert!(!timecard1.is_clocked_out());

        let entries2 = vec![
            TimeEntry {
                start: time - Duration::hours(8),
                end: Some(time - Duration::hours(4)),
            },
        ];
        let mut timecard2 = Timecard::new(entries2.clone())?;
        assert_eq!(entries2, timecard2.entries);
        assert!(!timecard2.is_clocked_in());
        assert!(timecard2.is_clocked_out());

        timecard2.clock_in(&time)?;

        assert_eq!(
            vec![
                entries2[0].clone(),
                TimeEntry {
                    start: time.clone(),
                    end: None,
                },
            ],
            timecard2.entries,
        );
        assert!(timecard2.is_clocked_in());
        assert!(!timecard2.is_clocked_out());

        let entries3 = vec![
            TimeEntry {
                start: time - Duration::hours(4),
                end: None,
            },
        ];
        let mut timecard3 = Timecard::new(entries3.clone())?;
        assert_eq!(entries3, timecard3.entries);
        assert!(timecard3.is_clocked_in());
        assert!(!timecard3.is_clocked_out());

        let result3 = timecard3.clock_in(&time);
        // TODO: Assert specific error
        assert!(result3.is_err());

        let result4 = timecard3.clock_in(&(Local::now() + Duration::seconds(1)));
        // TODO: Assert specific error
        assert!(result4.is_err());

        Ok(())
    }

    #[test]
    fn it_clocks_out() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();

        let mut timecard1 = Timecard::new(vec![])?;
        assert!(timecard1.entries.is_empty());
        assert!(!timecard1.is_clocked_in());
        assert!(timecard1.is_clocked_out());

        let result1 = timecard1.clock_out(&time);
        // TODO: Assert specific error
        assert!(result1.is_err());

        let entries2 = vec![
            TimeEntry {
                start: time - Duration::hours(4),
                end: None,
            },
        ];
        let mut timecard2 = Timecard::new(entries2.clone())?;
        assert_eq!(entries2, timecard2.entries);
        assert!(timecard2.is_clocked_in());
        assert!(!timecard2.is_clocked_out());

        timecard2.clock_out(&time)?;
        assert_eq!(
            vec![
                TimeEntry {
                    start: entries2[0].clone().start,
                    end: Some(time.clone()),
                },
            ],
            timecard2.entries,
        );
        assert!(!timecard2.is_clocked_in());
        assert!(timecard2.is_clocked_out());

        let entries3 = vec![
            TimeEntry {
                start: time - Duration::hours(8),
                end: Some(time - Duration::hours(5)),
            },
            TimeEntry {
                start: time - Duration::hours(4),
                end: None,
            },
        ];
        let mut timecard3 = Timecard::new(entries3.clone())?;
        assert_eq!(entries3, timecard3.entries);
        assert!(timecard3.is_clocked_in());
        assert!(!timecard3.is_clocked_out());

        timecard3.clock_out(&time)?;
        assert_eq!(
            vec![
                timecard3.entries[0].clone(),
                TimeEntry {
                    start: timecard3.entries[1].clone().start,
                    end: Some(time.clone()),
                },
            ],
            timecard3.entries,
        );

        Ok(())
    }

    #[test]
    fn it_undos() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        todo!();

        Ok(())
    }

    #[test]
    fn it_gets_duration_worked() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        todo!();

        Ok(())
    }

    #[test]
    fn it_gets_duration_on_break() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        todo!();

        Ok(())
    }

    #[test]
    fn it_gets_expected_end_time() -> Result<(), Box<dyn std::error::Error>> {
        let time = get_ref_time();
        todo!();

        Ok(())
    }
}
