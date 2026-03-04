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
    Timecard::new(vec![])?;
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
    
    let mut timecard1 = Timecard::new(vec![])?;
    assert!(timecard1.entries.is_empty());
    assert!(!timecard1.is_clocked_in());
    assert!(timecard1.is_clocked_out());

    let result = timecard1.undo();
    // TODO: Assert specific error
    assert!(result.is_err());

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

    timecard2.undo()?;
    assert!(timecard2.entries.is_empty());
    assert!(!timecard2.is_clocked_in());
    assert!(timecard2.is_clocked_out());

    let entries3 = vec![
        TimeEntry {
            start: time - Duration::hours(4),
            end: Some(time - Duration::hours(1)),
        },
    ];
    let mut timecard3 = Timecard::new(entries3.clone())?;
    assert_eq!(entries3, timecard3.entries);
    assert!(!timecard3.is_clocked_in());
    assert!(timecard3.is_clocked_out());

    timecard3.undo()?;
    assert_eq!(
        vec![
            TimeEntry {
                start: entries3[0].clone().start,
                end: None,
            },
        ],
        timecard3.entries,
    );
    assert!(timecard3.is_clocked_in());
    assert!(!timecard3.is_clocked_out());

    let entries4 = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(4)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: Some(time - Duration::minutes(5)),
        },
    ];
    let mut timecard4 = Timecard::new(entries4.clone())?;
    assert_eq!(entries4, timecard4.entries);
    assert!(!timecard4.is_clocked_in());
    assert!(timecard4.is_clocked_out());

    timecard4.undo()?;
    assert_eq!(
        vec![
            entries4[0].clone(),
            TimeEntry {
                start: entries4[1].clone().start,
                end: None,
            },
        ],
        timecard4.entries,
    );
    assert!(timecard4.is_clocked_in());
    assert!(!timecard4.is_clocked_out());

    let entries5 = vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(4)),
        },
        TimeEntry {
            start: time - Duration::hours(3),
            end: None,
        },
    ];
    let mut timecard5 = Timecard::new(entries5.clone())?;
    assert_eq!(entries5, timecard5.entries);
    assert!(timecard5.is_clocked_in());
    assert!(!timecard5.is_clocked_out());

    timecard5.undo()?;
    assert_eq!(vec![entries5[0].clone()], timecard5.entries);
    assert!(!timecard5.is_clocked_in());
    assert!(timecard5.is_clocked_out());

    Ok(())
}

fn get_duration_timecards(now: DateTime<Local>) -> Result<Vec<Timecard>, Box<dyn std::error::Error>> {
    let time = get_ref_time();

    let timecard0 = Timecard::new(
        vec![
            TimeEntry {
                start: time - Duration::hours(8),
                end: Some(time - Duration::hours(4)),
            },
        ]
    )?;

    let timecard1 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::hours(8),
            end: Some(time - Duration::hours(5)),
        },
        TimeEntry {
            start: time - Duration::hours(4),
            end: Some(time.clone()),
        },
    ])?;

    let timecard2 = Timecard::new(vec![
        TimeEntry {
            start: now - Duration::hours(8),
            end: Some(now - Duration::minutes(15)),
        },
        TimeEntry {
            start: now - Duration::minutes(10),
            end: None,
        },
    ])?;

    let timecard3 = Timecard::new(vec![
        TimeEntry {
            start: time - Duration::days(1) - Duration::hours(8),
            end: Some(time - Duration::hours(1)),
        },
    ])?;

    let timecard4 = Timecard::new(vec![
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

    let timecard5 = Timecard::new(vec![
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
            end: Some(time - Duration::hours(4)),
        },
        TimeEntry {
            start: now - Duration::hours(3),
            end: None,
        },
    ])?;

    let timecard6 = Timecard::new(vec![
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

    let timecard7 = Timecard::new(vec![
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

    Ok(vec![
        timecard0,
        timecard1,
        timecard2,
        timecard3,
        timecard4,
        timecard5,
        timecard6,
        timecard7,
    ])
}

#[test]
fn it_gets_duration_worked() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Local::now();
    let timecards = get_duration_timecards(now.clone())?;
    
    assert_eq!(Duration::hours(4), timecards[0].get_duration_worked(&time, true));
    assert_eq!(Duration::hours(4), timecards[0].get_duration_worked(&time, false));

    assert_eq!(Duration::hours(7), timecards[1].get_duration_worked(&time, true));
    assert_eq!(Duration::hours(7), timecards[1].get_duration_worked(&time, false));

    assert_eq!(Duration::hours(8) - Duration::minutes(5), timecards[2].get_duration_worked(&now, true));
    assert_eq!(Duration::hours(8) - Duration::minutes(15), timecards[2].get_duration_worked(&now, false));

    assert_eq!(Duration::days(1) + Duration::hours(7), timecards[3].get_duration_worked(&time, true));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecards[3].get_duration_worked(&time, false));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecards[3].get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::days(1) + Duration::hours(7), timecards[3].get_duration_worked(&(time - Duration::days(1)), false));

    assert_eq!(Duration::hours(8), timecards[4].get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(8), timecards[4].get_duration_worked(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(9), timecards[4].get_duration_worked(&time, true));
    assert_eq!(Duration::hours(9), timecards[4].get_duration_worked(&time, false));

    assert_eq!(Duration::hours(8), timecards[5].get_duration_worked(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(8), timecards[5].get_duration_worked(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(9), timecards[5].get_duration_worked(&time, true));
    assert_eq!(Duration::hours(9), timecards[5].get_duration_worked(&time, false));
    assert_eq!(Duration::hours(7), timecards[5].get_duration_worked(&now, true));
    assert_eq!(Duration::hours(4), timecards[5].get_duration_worked(&now, false));

    assert_eq!(Duration::minutes(16), timecards[6].get_duration_worked(&now, true));
    assert_eq!(Duration::minutes(15), timecards[6].get_duration_worked(&now, false));

    assert_eq!(Duration::minutes(17), timecards[7].get_duration_worked(&now, true));
    assert_eq!(Duration::minutes(17), timecards[7].get_duration_worked(&now, false));

    Ok(())
}

#[test]
fn it_gets_duration_on_break() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Local::now();
    let timecards = get_duration_timecards(now.clone())?;
    
    assert_eq!(Duration::zero(), timecards[0].get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecards[0].get_duration_on_break(&time, false));

    assert_eq!(Duration::hours(1), timecards[1].get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(1), timecards[1].get_duration_on_break(&time, false));

    assert_eq!(Duration::minutes(5), timecards[2].get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(5), timecards[2].get_duration_on_break(&now, false));
    assert_eq!(Duration::zero(), timecards[2].get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecards[2].get_duration_on_break(&time, false));

    assert_eq!(Duration::zero(), timecards[3].get_duration_on_break(&time, true));
    assert_eq!(Duration::zero(), timecards[3].get_duration_on_break(&time, false));
    assert_eq!(Duration::zero(), timecards[3].get_duration_on_break(&now, true));
    assert_eq!(Duration::zero(), timecards[3].get_duration_on_break(&now, false));

    assert_eq!(Duration::hours(1), timecards[4].get_duration_on_break(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(1), timecards[4].get_duration_on_break(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(2), timecards[4].get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(2), timecards[4].get_duration_on_break(&time, false));

    assert_eq!(Duration::hours(1), timecards[5].get_duration_on_break(&(time - Duration::days(1)), true));
    assert_eq!(Duration::hours(1), timecards[5].get_duration_on_break(&(time - Duration::days(1)), false));
    assert_eq!(Duration::hours(2), timecards[5].get_duration_on_break(&time, true));
    assert_eq!(Duration::hours(2), timecards[5].get_duration_on_break(&time, false));
    assert_eq!(Duration::hours(1), timecards[5].get_duration_on_break(&now, true));
    assert_eq!(Duration::hours(1), timecards[5].get_duration_on_break(&now, false));

    assert_eq!(Duration::minutes(14), timecards[6].get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(14), timecards[6].get_duration_on_break(&now, false));

    assert_eq!(Duration::minutes(13), timecards[7].get_duration_on_break(&now, true));
    assert_eq!(Duration::minutes(11), timecards[7].get_duration_on_break(&now, false));

    Ok(())
}

#[test]
fn it_gets_expected_end_time() -> Result<(), Box<dyn std::error::Error>> {
    let time = get_ref_time();
    let now = Local::now();
    let timecards = get_duration_timecards(now.clone())?;
    
    assert_eq!(time, timecards[0].get_expected_end_time(&Duration::hours(8), &time));
    
    assert_eq!(time + Duration::hours(1), timecards[1].get_expected_end_time(&Duration::hours(8), &time));
    assert_eq!(time, timecards[1].get_expected_end_time(&Duration::hours(7), &time));

    assert_eq!(now + Duration::minutes(5), timecards[2].get_expected_end_time(&Duration::hours(8), &now));

    assert_eq!(time - Duration::days(1), timecards[3].get_expected_end_time(&Duration::hours(8), &(time - Duration::days(1))));

    Ok(())
}
