use chrono::{Duration, NaiveDate, Weekday, WeekdaySet};

use super::*;
use crate::serializable::SerializableWeekdaySet;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn mon_fri_requirements() -> WorkPeriodRequirements {
    let weekdays = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
    ]
    .into_iter()
    .collect::<WeekdaySet>();
    WorkPeriodRequirements {
        work_days_of_week: SerializableWeekdaySet(weekdays),
        work_day_duration: Duration::hours(8),
        exempt_days: vec![],
        round_durations_to: Duration::minutes(15),
    }
}

// ---------------------------------------------------------------------------
// TimePeriod::new / validate
// ---------------------------------------------------------------------------

#[test]
fn new_accepts_valid_period() {
    let period = TimePeriod::new(date(2024, 1, 1), date(2024, 1, 31)).unwrap();
    assert_eq!(period.start(), date(2024, 1, 1));
    assert_eq!(period.end(), date(2024, 1, 31));
}

#[test]
fn new_accepts_single_day_period() {
    let d = date(2024, 6, 15);
    let period = TimePeriod::new(d, d).unwrap();
    assert_eq!(period.start(), d);
    assert_eq!(period.end(), d);
}

#[test]
fn new_rejects_inverted_period() {
    let err = TimePeriod::new(date(2024, 1, 31), date(2024, 1, 1)).unwrap_err();
    assert!(
        matches!(err, ValidationError::InvertedTime),
        "Expected ValidationError::InvertedTime, got {err:?}",
    );
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

#[test]
fn display_formats_correctly() {
    let period = TimePeriod::new(date(2024, 1, 1), date(2024, 1, 31)).unwrap();
    assert_eq!(period.to_string(), "2024-01-01 - 2024-01-31");
}

// ---------------------------------------------------------------------------
// estimate_time_to_work
// ---------------------------------------------------------------------------

// TODO: Potential bug to check for:
// If user works for just a few minutes less, but still rounds up to nearest tenth/quarter/half/whole hour (call these ticks),
// I could definitely see the system saying the next day should have one less tick,
// since there's extra time, but the extra time isn't large enough to cover an entire tick.

#[test]
fn estimate_time_to_work_past_period_uses_timecard_entries() {
    todo!()
}

#[test]
fn estimate_time_to_work_future_period_returns_full_day_durations() {
    todo!()
}

#[test]
fn estimate_time_to_work_distributes_deficit_across_future_days() {
    todo!()
}

#[test]
fn estimate_time_to_work_exempt_days_do_not_contribute_to_deficit() {
    todo!()
}

#[test]
fn estimate_time_to_work_duration_less_than_tick_does_not_contribute() {
    todo!()
}
