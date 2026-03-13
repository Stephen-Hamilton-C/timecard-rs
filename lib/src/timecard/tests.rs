use crate::error::ClockState;

use super::*;

fn get_ref_time() -> DateTime<Utc> {
    DateTime::from_timestamp_millis(1314129600000).unwrap()
}

#[test]
fn rejects_bad_entry() {
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
    let err = Timecard::new(entries).unwrap_err();
    assert!(
        matches!(err, ValidationError::InvertedEntry),
        "Expected ValidationError::InvertedEntry, got {err:?}",
    );
}

#[test]
fn rejects_bad_order() {
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
    let err = Timecard::new(entries).unwrap_err();
    assert!(
        matches!(err, ValidationError::Chronological),
        "Expected ValidationError::Chronological, got {err:?}"
    );
}

#[test]
fn rejects_bad_none_end_entry_order() {
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
    let err = Timecard::new(entries).unwrap_err();
    assert!(
        matches!(err, ValidationError::EndTimeRequired),
        "Expected ValidationError::EndTimeRequired, got {err:?}",
    );
}

#[test]
fn rejects_overlapping_entries_out_of_order() {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time + Duration::minutes(5),
            end: Some(time + Duration::minutes(10)),
        },
        TimeEntry {
            start: time + Duration::minutes(3),
            end: Some(time + Duration::minutes(12)),
        },
    ];
    let err = Timecard::new(entries).unwrap_err();
    assert!(
        matches!(err, ValidationError::Chronological),
        "Expected ValidationError::Chronological, got {err:?}",
    );
}

#[test]
fn rejects_overlapping_entries_contained() {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time + Duration::minutes(3),
            end: Some(time + Duration::minutes(12)),
        },
        TimeEntry {
            start: time + Duration::minutes(5),
            end: Some(time + Duration::minutes(10)),
        },
    ];
    let err = Timecard::new(entries).unwrap_err();
    assert!(
        matches!(err, ValidationError::Chronological),
        "Expected ValidationError::Chronological, got {err:?}",
    )
}

#[test]
fn creates_empty_timecard() -> Result<(), Box<dyn std::error::Error>> {
    Timecard::new(vec![])?;
    Ok(())
}

#[test]
fn creates_timecard_with_consecutive_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
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
    Ok(())
}

#[test]
fn creates_timecard_with_single_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    Timecard::new(vec![
        TimeEntry {
            start: time - Duration::minutes(10),
            end: Some(time - Duration::minutes(5)),
        },
    ])?;
    Ok(())
}

#[test]
fn creates_timecard_with_closed_and_open_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
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
    Ok(())
}

#[test]
fn creates_timecard_with_single_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    Timecard::new(vec![
        TimeEntry {
            start: time - Duration::minutes(1),
            end: None,
        },
    ])?;
    Ok(())
}

#[test]
fn creates_timecard_with_zero_duration_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    Timecard::new(vec![
        TimeEntry {
            start: time,
            end: Some(time),
        },
    ])?;
    Ok(())
}

#[test]
fn gets_entries() -> Result<(), Box<dyn std::error::Error>> {
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
fn reports_clocked_out_when_empty() -> Result<(), Box<dyn std::error::Error>> {
    let timecard = Timecard::new(vec![])?;
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());
    Ok(())
}

#[test]
fn reports_clocked_in_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let timecard = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::minutes(10),
            end: None,
        },
    ])?;
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn reports_clocked_out_with_closed_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let timecard = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::minutes(10),
            end: Some(time - Duration::minutes(5)),
        },
        TimeEntry {
            start: time - Duration::minutes(3),
            end: Some(time - Duration::minutes(1)),
        },
    ])?;
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());
    Ok(())
}

#[test]
fn reports_clocked_in_with_closed_and_open_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let timecard = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::minutes(10),
            end: Some(time - Duration::minutes(5)),
        },
        TimeEntry {
            start: time - Duration::minutes(3),
            end: None,
        },
    ])?;
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn filters_by_day_empty_timecard() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let timecard = Timecard::new(vec![])?;
    assert!(timecard.filter_by_day(&time).is_empty());
    assert!(timecard.filter_by_day(&(time - Duration::days(1))).is_empty());
    assert!(timecard.filter_by_day(&(time - Duration::weeks(52))).is_empty());
    Ok(())
}

#[test]
fn filters_by_day_single_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::minutes(10),
            end: None,
        },
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_day(&time));
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_day(&(time - Duration::minutes(30))));
    assert!(timecard.filter_by_day(&(time - Duration::days(1))).is_empty());
    assert!(timecard.filter_by_day(&(time - Duration::weeks(52))).is_empty());
    Ok(())
}

#[test]
fn filters_by_day_single_closed_past_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::days(3) - Duration::minutes(10),
            end: Some(time - Duration::days(3)),
        }
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_day(&(time - Duration::days(3))));
    assert!(timecard.filter_by_day(&(time - Duration::days(2))).is_empty());
    assert!(timecard.filter_by_day(&(time - Duration::days(4))).is_empty());
    assert!(timecard.filter_by_day(&(time - Duration::days(1))).is_empty());
    assert!(timecard.filter_by_day(&time).is_empty());
    Ok(())
}

#[test]
fn filters_by_day_multiple_entries_on_different_days() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
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
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(3))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&(time - Duration::days(2))));
    assert_eq!(vec![&entries[2]], timecard.filter_by_day(&(time - Duration::days(1))));
    assert!(timecard.filter_by_day(&(time - Duration::days(4))).is_empty());
    assert!(timecard.filter_by_day(&time).is_empty());
    Ok(())
}

#[test]
fn filters_by_day_entry_spanning_two_days() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::days(3) - Duration::minutes(10),
            end: Some(time - Duration::days(3)),
        },
        TimeEntry {
            start: time - Duration::days(2) - Duration::minutes(10),
            end: Some(time - Duration::days(1)),
        },
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(3))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&(time - Duration::days(2))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&(time - Duration::days(1))));
    assert!(timecard.filter_by_day(&(time - Duration::days(4))).is_empty());
    assert!(timecard.filter_by_day(&time).is_empty());
    Ok(())
}

#[test]
fn filters_by_day_open_entry_spanning_multiple_days() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::days(3) - Duration::minutes(10),
            end: Some(time - Duration::days(3)),
        },
        TimeEntry {
            start: time - Duration::days(2) - Duration::minutes(10),
            end: None,
        },
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(3))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&(time - Duration::days(2))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&(time - Duration::days(1))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_day(&time));
    Ok(())
}

#[test]
fn filters_by_day_multi_day_entry_with_separate_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::days(3),
            end: Some(time.clone()),
        },
        TimeEntry {
            start: time + Duration::days(1),
            end: None,
        },
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(3))));
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(2))));
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&(time - Duration::days(1))));
    assert_eq!(vec![&entries[0]], timecard.filter_by_day(&time));
    Ok(())
}

#[test]
fn filters_by_date_range_multiple_closed_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
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
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(vec![&entries[0]], timecard.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(14))));
    assert_eq!(vec![&entries[1]], timecard.filter_by_date_range(&(time - Duration::days(13)), &(time - Duration::days(13))));
    assert_eq!(
        vec![
            &entries[0],
            &entries[1],
        ],
        timecard.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(13))),
    );
    assert_eq!(
        vec![
            &entries[0],
            &entries[1],
            &entries[2],
            &entries[3],
        ],
        timecard.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(12))),
    );
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_date_range(&(time - Duration::days(14)), &(time - Duration::days(8))));
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_date_range(&(time - Duration::days(20)), &time));
    assert_eq!(vec![&entries[4]], timecard.filter_by_date_range(&(time - Duration::days(10)), &(time - Duration::days(8))));
    Ok(())
}

#[test]
fn filters_by_date_range_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::days(2),
            end: None,
        },
    ];
    let timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_date_range(&(time - Duration::days(2)), &Utc::now()));
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_date_range(&(time - Duration::days(2)), &(time - Duration::days(2))));
    assert_eq!(entries.iter().collect::<Vec<_>>(), timecard.filter_by_date_range(&(time - Duration::days(3)), &Utc::now()));
    Ok(())
}

#[test]
fn clears_nonempty_timecard() -> Result<(), Box<dyn std::error::Error>> {
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
fn clears_empty_timecard() -> Result<(), Box<dyn std::error::Error>> {
    let mut timecard = Timecard::new(vec![])?;

    assert!(timecard.entries.is_empty());
    timecard.clear();
    assert!(timecard.entries.is_empty());
    Ok(())
}

#[test]
fn clocks_in_when_empty() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let mut timecard = Timecard::new(vec![])?;
    assert!(timecard.entries.is_empty());
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    timecard.clock_in(time.clone())?;

    assert_eq!(
        vec![TimeEntry { start: time, end: None }],
        timecard.entries,
    );
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn clocks_in_when_clocked_out_with_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(4)),
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    timecard.clock_in(time.clone())?;

    assert_eq!(
        vec![
            entries[0].clone(),
            TimeEntry {
                start: time.clone(),
                end: None,
            },
        ],
        timecard.entries,
    );
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn clock_in_errors_when_already_clocked_in() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(4),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    let err = timecard.clock_in(time.clone()).unwrap_err();
    assert!(
        matches!(err, ClockError::AlreadyInState(ClockState::In)),
        "Expected ClockError::AlreadyInState(ClockState::In), got {err:?}",
    );

    Ok(())
}

#[test]
fn clock_in_errors_when_time_in_future() -> Result<(), Box<dyn std::error::Error>> {
    let mut timecard = Timecard::new(vec![])?;
    let err = timecard.clock_in(Utc::now() + Duration::seconds(1)).unwrap_err();
    assert!(
        matches!(err, ClockError::TimeInFuture),
        "Expected ClockError::TimeInFuture, got {err:?}",
    );

    Ok(())
}

#[test]
fn clock_out_errors_when_not_clocked_in() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let mut timecard = Timecard::new(vec![])?;
    assert!(timecard.entries.is_empty());
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    let err = timecard.clock_out(time.clone()).unwrap_err();
    assert!(
        matches!(err, ClockError::AlreadyInState(ClockState::Out)),
        "Expected ClockError::AlreadyInState(ClockState::Out), got {err:?}",
    );
    Ok(())
}

#[test]
fn clocks_out_when_clocked_in() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(4),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    timecard.clock_out(time.clone())?;
    assert_eq!(
        vec![
            TimeEntry {
                start: entries[0].clone().start,
                end: Some(time.clone()),
            },
        ],
        timecard.entries,
    );
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    let err = timecard.clock_out(time.clone()).unwrap_err();
    assert!(
        matches!(err, ClockError::AlreadyInState(ClockState::Out)),
        "Expected ClockError::AlreadyInState(ClockState::Out)), got {err:?}",
    );
    Ok(())
}

#[test]
fn clocks_out_with_multiple_entries() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::hours(4),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    timecard.clock_out(time.clone())?;
    assert_eq!(
        vec![
            timecard.entries[0].clone(),
            TimeEntry {
                start: timecard.entries[1].clone().start,
                end: Some(time.clone()),
            },
        ],
        timecard.entries,
    );
    Ok(())
}

#[test]
fn clock_out_errors_with_invalid_times() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let entries = vec![
        TimeEntry {
            start: now - Duration::hours(4),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    let err = timecard.clock_out(now + Duration::hours(1)).unwrap_err();
    assert!(
        matches!(err, ClockError::TimeInFuture),
        "Expected ClockError::TimeInFuture, got {err:?}",
    );

    let err = timecard.clock_out(now - Duration::hours(5)).unwrap_err();
    assert!(
        matches!(err, ClockError::BeforeLastEntry),
        "Expected ClockError::BeforeLastEntry, got {err:?}",
    );
    Ok(())
}

#[test]
fn undo_errors_when_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut timecard = Timecard::new(vec![])?;
    assert!(timecard.entries.is_empty());
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    let err = timecard.undo().unwrap_err();
    assert!(
        matches!(err, UndoError::EmptyEntries),
        "Expected UndoError::EmptyEntries, got {err:?}",
    );
    Ok(())
}

#[test]
fn undo_removes_single_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(4),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    timecard.undo()?;
    assert!(timecard.entries.is_empty());
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());
    Ok(())
}

#[test]
fn undo_reopens_single_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(4),
            end: Some(time - Duration::hours(1)),
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    timecard.undo()?;
    assert_eq!(
        vec![
            TimeEntry {
                start: entries[0].clone().start,
                end: None,
            },
        ],
        timecard.entries,
    );
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn undo_reopens_last_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(4)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: Some(time - Duration::minutes(5)),
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());

    timecard.undo()?;
    assert_eq!(
        vec![
            entries[0].clone(),
            TimeEntry {
                start: entries[1].clone().start,
                end: None,
            },
        ],
        timecard.entries,
    );
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());
    Ok(())
}

#[test]
fn undo_removes_open_entry_with_prior_closed() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let entries = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(4)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: None,
        },
    ];
    let mut timecard = Timecard::new(entries.clone())?;
    assert_eq!(entries, timecard.entries);
    assert!(timecard.is_clocked_in());
    assert!(!timecard.is_clocked_out());

    timecard.undo()?;
    assert_eq!(vec![entries[0].clone()], timecard.entries);
    assert!(!timecard.is_clocked_in());
    assert!(timecard.is_clocked_out());
    Ok(())
}

fn get_duration_timecards(now: DateTime<Utc>) -> Result<Vec<Timecard>, Box<dyn std::error::Error>> {
    let time = get_ref_time();

    let mut timecard0 = Timecard::new(
        vec![
            TimeEntry {
                start: time - Duration::hours(8),
                end: Some(time - Duration::hours(4)),
            },
        ]
    )?;

    let mut timecard1 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::hours(4),
            end: Some(time.clone()),
        },
    ])?;

    let mut timecard2 = Timecard::new(vec![
        TimeEntry {
            start: now - Duration::hours(8),
            end: Some(now - Duration::minutes(15)),
        },
        TimeEntry {
            start: now - Duration::minutes(10),
            end: None,
        },
    ])?;

    let mut timecard3 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(8),
            end: Some(time - Duration::hours(1)),
        },
    ])?;

    let mut timecard4 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(9),
            end: Some(time - Duration::days(1) - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(4),
            end: Some(time - Duration::days(1)),
        },
        TimeEntry {
            start: time - Duration::hours(9),
            end: Some(time - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: Some(time.clone()),
        },
    ])?;

    let mut timecard5 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(9),
            end: Some(time - Duration::days(1) - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(4),
            end: Some(time - Duration::days(1)),
        },
        TimeEntry {
            start: time - Duration::hours(9),
            end: Some(time - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: Some(time.clone()),
        },
        TimeEntry {
            start: now - Duration::hours(8),
            end: Some(now - Duration::hours(4)),
        },
        TimeEntry {
            start: now - Duration::hours(3),
            end: None,
        },
    ])?;

    let mut timecard6 = Timecard::new(vec![
        TimeEntry {
            start: now - Duration::minutes(30),
            end: Some(now - Duration::minutes(25)),
        },
        TimeEntry {
            start: now - Duration::minutes(15),
            end: Some(now - Duration::minutes(5)),
        },
        TimeEntry {
            start: now - Duration::minutes(1),
            end: None
        },
    ])?;

    let mut timecard7 = Timecard::new(vec![
        TimeEntry {
            start: now - Duration::minutes(30),
            end: Some(now - Duration::minutes(25)),
        },
        TimeEntry {
            start: now - Duration::minutes(15),
            end: Some(now - Duration::minutes(5)),
        },
        TimeEntry {
            start: now - Duration::minutes(4),
            end: Some(now - Duration::minutes(2)),
        },
    ])?;

    let mut timecard8 = Timecard::new(vec![])?;

    timecard0.now_override = Some(now);
    timecard1.now_override = Some(now);
    timecard2.now_override = Some(now);
    timecard3.now_override = Some(now);
    timecard4.now_override = Some(now);
    timecard5.now_override = Some(now);
    timecard6.now_override = Some(now);
    timecard7.now_override = Some(now);
    timecard8.now_override = Some(now);

    Ok(vec![
        timecard0,
        timecard1,
        timecard2,
        timecard3,
        timecard4,
        timecard5,
        timecard6,
        timecard7,
        timecard8,
    ])
}

#[test]
fn gets_duration_worked_single_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[0];
    assert_eq!(Duration::hours(4), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::hours(4), timecard.get_duration_worked(&time, false));
    Ok(())
}

#[test]
fn gets_duration_worked_two_closed_entries_with_break() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[1];
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, false));
    Ok(())
}

#[test]
fn gets_duration_worked_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[2];
    assert_eq!(Duration::hours(8) - Duration::minutes(5), timecard.get_duration_worked(&now, true));
    assert_eq!(Duration::hours(8) - Duration::minutes(15), timecard.get_duration_worked(&now, false));
    Ok(())
}

#[test]
fn gets_duration_worked_multi_day_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[3];
    assert_eq!(Duration::days(1) + Duration::hours(7), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecard.get_duration_worked(&time, false));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecard.get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecard.get_duration_worked(&(time - Duration::days(1)), false));
    Ok(())
}

#[test]
fn gets_duration_worked_multi_day_multi_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[4];
    assert_eq!(Duration::hours(8), timecard.get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(8), timecard.get_duration_worked(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, false));
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&(time - Duration::weeks(4)), true));
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&(time - Duration::weeks(4)), false));
    Ok(())
}

#[test]
fn gets_duration_worked_multi_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[5];
    assert_eq!(Duration::hours(8), timecard.get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(8), timecard.get_duration_worked(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&time, false));
    assert_eq!(Duration::hours(7), timecard.get_duration_worked(&now, true));
    assert_eq!(Duration::hours(4), timecard.get_duration_worked(&now, false));
    Ok(())
}

#[test]
fn gets_duration_worked_current_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[6];
    assert_eq!(Duration::minutes(16), timecard.get_duration_worked(&now, true));
    assert_eq!(Duration::minutes(15), timecard.get_duration_worked(&now, false));
    Ok(())
}

#[test]
fn gets_duration_worked_current_day_all_closed() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[7];
    assert_eq!(Duration::minutes(17), timecard.get_duration_worked(&now, true));
    assert_eq!(Duration::minutes(17), timecard.get_duration_worked(&now, false));
    Ok(())
}

#[test]
fn gets_duration_worked_empty_timecard() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[8];
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&time, true));
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&time, false));
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&now, true));
    assert_eq!(Duration::zero(), timecard.get_duration_worked(&now, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_single_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[0];
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_two_closed_entries_with_break() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[1];
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&time, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[2];
    assert_eq!(Duration::minutes(5), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(5), timecard.get_duration_on_break(&now, false));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_multi_day_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[3];
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, false));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&now, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_multi_day_multi_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[4];
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(2), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(2), timecard.get_duration_on_break(&time, false));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&(time - Duration::weeks(4)), true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&(time - Duration::weeks(4)), false));
    Ok(())
}

#[test]
fn gets_duration_on_break_multi_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[5];
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(2), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(2), timecard.get_duration_on_break(&time, false));
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::hours(1), timecard.get_duration_on_break(&now, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_current_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[6];
    assert_eq!(Duration::minutes(14), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(14), timecard.get_duration_on_break(&now, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_current_day_all_closed() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    println!("{}, {}", get_ref_time(), now);
    let timecard = &get_duration_timecards(now.clone())?[7];
    assert_eq!(Duration::minutes(13), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(11), timecard.get_duration_on_break(&now, false));
    Ok(())
}

#[test]
fn gets_duration_on_break_empty_timecard() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[8];
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&time, false));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&now, true));
    assert_eq!(Duration::zero(), timecard.get_duration_on_break(&now, false));
    Ok(())
}

// TODO: Need a test for None
#[test]
fn gets_expected_end_time_single_closed_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[0];
    assert_eq!(Some(time), timecard.get_expected_end_time(Duration::hours(8), &time));
    Ok(())
}

#[test]
fn gets_expected_end_time_two_closed_entries_with_break() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[1];
    assert_eq!(Some(time + Duration::hours(1)), timecard.get_expected_end_time(Duration::hours(8), &time));
    assert_eq!(Some(time), timecard.get_expected_end_time(Duration::hours(7), &time));
    assert_eq!(Some(time - Duration::hours(3)), timecard.get_expected_end_time(Duration::hours(4), &time));
    Ok(())
}

#[test]
fn gets_expected_end_time_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[2];
    assert_eq!(Some(now + Duration::minutes(5)), timecard.get_expected_end_time(Duration::hours(8), &now));
    assert_eq!(Some(now + Duration::minutes(5) - Duration::hours(1)), timecard.get_expected_end_time(Duration::hours(7), &now));
    Ok(())
}

#[test]
fn gets_expected_end_time_multi_day_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[3];
    assert_eq!(Some(time - Duration::days(1)), timecard.get_expected_end_time(Duration::hours(8), &(time - Duration::days(1))));
    Ok(())
}

#[test]
fn gets_expected_end_time_multi_day_multi_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now)?[4];
    assert_eq!(Some(time - Duration::days(1)), timecard.get_expected_end_time(Duration::hours(8), &(time - Duration::days(1))));
    assert_eq!(Some(time + Duration::hours(1)), timecard.get_expected_end_time(Duration::hours(8), &time));
    Ok(())
}

#[test]
fn gets_expected_end_time_multi_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[5];
    assert_eq!(Some(time - Duration::days(1)), timecard.get_expected_end_time(Duration::hours(8), &(time - Duration::days(1))));
    assert_eq!(Some(time + Duration::hours(1)), timecard.get_expected_end_time(Duration::hours(8), &time));
    assert_eq!(Some(now + Duration::hours(1)), timecard.get_expected_end_time(Duration::hours(8), &now));
    Ok(())
}

#[test]
fn gets_expected_end_time_current_day_with_open_entry() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[6];
    assert_eq!(Some(now + Duration::hours(4) - Duration::minutes(16)), timecard.get_expected_end_time(Duration::hours(4), &now));
    Ok(())
}

#[test]
fn gets_expected_end_time_current_day_all_closed() -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let timecard = &get_duration_timecards(now.clone())?[7];
    assert_eq!(Some(now + Duration::hours(2) - Duration::minutes(17)), timecard.get_expected_end_time(Duration::hours(2), &now));
    Ok(())
}
