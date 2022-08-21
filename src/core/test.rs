use chrono::{Duration, Utc, SubsecRound};
use std::ops::Sub;

use super::*;

#[test]
fn timeentry_serializes() {
    let start = Utc::now().naive_local();
    let entry = TimeEntry {
        start: start.clone(),
        end: None,
    };
    let entry_data: JsonValue = entry.into();
    let expected_data = object! {
        start: start.timestamp(),
        end: null,
    };

    assert_eq!(entry_data, expected_data);
}

#[test]
fn timeentry_deserializes() {
    let start = Utc::now().naive_local();
    let entry_data = object! {
        start: start.timestamp(),
        end: null,
    };
    let entry = TimeEntry::from(&entry_data);
    let expected_entry = TimeEntry { start, end: None };

    assert_eq!(entry, expected_entry);
}

#[test]
fn timeentries_serializes() {
    let now = Utc::now().naive_local();
    let entry1 = TimeEntry {
        start: now.sub(Duration::minutes(30)),
        end: Some(now.sub(Duration::minutes(15))),
    };
    let entry2 = TimeEntry {
        start: now.sub(Duration::minutes(5)),
        end: None,
    };

    let entries = TimeEntries { entries: vec![entry1.clone(), entry2.clone()] };
    let entries_data: JsonValue = entries.into();
    let expected_data = object! {
        entries: [
            {
                start: entry1.start.timestamp(),
                end: entry1.end.unwrap().timestamp(),
            },
            {
                start: entry2.start.timestamp(),
                end: null,
            },
        ]
    };

    assert_eq!(entries_data, expected_data);
}

#[test]
fn timeentries_deserializes() {
    let now = Utc::now().naive_local().round_subsecs(0);
    let entry1 = TimeEntry {
        start: now.sub(Duration::minutes(30)),
        end: Some(now.sub(Duration::minutes(15))),
    };
    let entry2 = TimeEntry {
        start: now.sub(Duration::minutes(5)),
        end: None,
    };

    let entries_data = object! {
        entries: [
            {
                start: entry1.start.timestamp(),
                end: entry1.end.unwrap().timestamp(),
            },
            {
                start: entry2.start.timestamp(),
                end: null,
            },
        ]
    };
    let entries = TimeEntries::from(&entries_data);

    assert_eq!(entries.entries[0].start, entry1.start);
    assert_eq!(entries.entries[0].end, entry1.end);
    assert_eq!(entries.entries[1].start, entry2.start);
    assert_eq!(entries.entries[1].end, entry2.end);
}
