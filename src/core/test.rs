use chrono::{Duration, Utc};
use std::ops::Sub;

use super::*;

#[test]
fn timeentry_serializes() {
    let start = Utc::now().naive_local();
    let entry = TimeEntry {
        start: start.clone(),
        end: None,
    };
    let entry_data = entry.serialize();
    let expected_data = object! {
        start: start.timestamp(),
        end: null,
    }.dump();

    assert_eq!(entry_data, expected_data);
}

#[test]
fn timeentry_deserializes() {
    let start = Utc::now().naive_local();
    let entry_data = object! {
        start: start.timestamp(),
        end: null,
    }.dump();
    let entry = TimeEntry::from(entry_data);
    let expected_entry = TimeEntry { start, end: None };

    assert_eq!(entry, expected_entry);
}

/*
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

    let entries = TimeEntries::new(vec![entry1, entry2]);
    let entries_data = timeEntries.serialize();
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
    }.dump();

    assert_eq!(entries_data, expected_data);
}

#[test]
fn timeentries_deserializes() {

}
*/