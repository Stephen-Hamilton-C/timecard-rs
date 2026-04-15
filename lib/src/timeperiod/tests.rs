use chrono::{DateTime, Duration, NaiveDate, NaiveTime, Weekday, WeekdaySet};

use super::*;
use crate::{TimeEntry, serializable::SerializableWeekdaySet};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn datetime(date: NaiveDate, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
    date.and_time(NaiveTime::from_hms_opt(hour, min, sec).unwrap())
        .and_utc()
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

fn assert_dur_opt_eq(dur1: Option<Duration>, dur2: Option<Duration>, day: NaiveDate) {
    assert_eq!(dur1, dur2, "expected different duration for day {}", day);
}

fn assert_dur_eq(dur1: Duration, dur2: Duration, day: NaiveDate) {
    assert_dur_opt_eq(Some(dur1), Some(dur2), day);
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
// get_dates
// ---------------------------------------------------------------------------

#[test]
fn get_dates_returns_all_dates() {
    let period = TimePeriod::new(date(2024, 1, 1), date(2024, 1, 31)).unwrap();
    for (i, day) in period.get_dates().iter().enumerate() {
        assert_eq!(date(2024, 1, (i + 1).try_into().unwrap()), *day);
    }

    let period = TimePeriod::new(date(2025, 2, 10), date(2025, 2, 17)).unwrap();
    for (i, day) in period.get_dates().iter().enumerate() {
        assert_eq!(date(2025, 2, (i + 10).try_into().unwrap()), *day);
    }
}

#[test]
fn get_dates_returns_single_date() {
    let period = TimePeriod::new(date(2024, 1, 1), date(2024, 1, 1)).unwrap();
    let dates = period.get_dates();
    assert_eq!(dates.len(), 1);
    assert_eq!(*dates.first().unwrap(), date(2024, 1, 1));
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
fn estimate_time_to_work_future_period_returns_full_day_durations()
-> Result<(), Box<dyn std::error::Error>> {
    let period_start = date(2026, 3, 30);
    let period_end = date(2026, 4, 3);
    let mut period = TimePeriod::new(period_start, period_end)?;
    let today = period_start + Duration::days(2);
    period.today_override = Some(today);
    let timecard = Timecard::new(vec![
        TimeEntry::new(
            datetime(period_start, 8, 0, 0),
            Some(datetime(period_start, 16, 0, 0)),
        )?,
        TimeEntry::new(
            datetime(period_start + Duration::days(1), 8, 0, 0),
            Some(datetime(period_start + Duration::days(1), 16, 0, 0)),
        )?,
        TimeEntry::new(
            datetime(period_start + Duration::days(2), 8, 0, 0),
            Some(datetime(period_start + Duration::days(2), 16, 0, 0)),
        )?,
    ])?;
    let reqs = mon_fri_requirements();
    let estimate = period.estimate_time_to_work(&timecard, &reqs);
    for (day, duration) in estimate {
        assert_dur_opt_eq(duration, Some(reqs.work_day_duration), day);
    }
    Ok(())
}

#[test]
fn estimate_time_to_work_distributes_deficit_across_future_days()
-> Result<(), Box<dyn std::error::Error>> {
    let period_start = date(2026, 3, 30);
    let period_end = date(2026, 4, 3);
    let mut period = TimePeriod::new(period_start, period_end)?;
    let today = period_start + Duration::days(2);
    period.today_override = Some(today);
    let timecard = Timecard::new(vec![
        TimeEntry::new(
            // 1 hour deficit
            datetime(period_start, 8, 0, 0),
            Some(datetime(period_start, 15, 0, 0)),
        )?,
        TimeEntry::new(
            // Full day
            datetime(period_start + Duration::days(1), 8, 0, 0),
            Some(datetime(period_start + Duration::days(1), 16, 0, 0)),
        )?,
        TimeEntry::new(
            // Full day
            datetime(period_start + Duration::days(2), 8, 0, 0),
            Some(datetime(period_start + Duration::days(2), 16, 0, 0)),
        )?,
    ])?;
    let reqs = mon_fri_requirements();
    let estimate = period.estimate_time_to_work(&timecard, &reqs);
    let day = date(2026, 3, 30);
    assert_dur_opt_eq(estimate[&day], Some(Duration::hours(7)), day);
    let day = date(2026, 3, 31);
    assert_dur_opt_eq(estimate[&day], Some(Duration::hours(8)), day);
    let day = date(2026, 4, 1);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) + Duration::minutes(30)),
        day,
    );
    let day = date(2026, 4, 2);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) + Duration::minutes(15)),
        day,
    );
    let day = date(2026, 4, 3);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) + Duration::minutes(15)),
        day,
    );
    Ok(())
}

#[test]
fn estimate_time_to_work_distributes_excess_across_future_days()
-> Result<(), Box<dyn std::error::Error>> {
    let period_start = date(2026, 3, 30);
    let period_end = date(2026, 4, 3);
    let mut period = TimePeriod::new(period_start, period_end)?;
    let today = period_start + Duration::days(2);
    period.today_override = Some(today);
    let timecard = Timecard::new(vec![
        TimeEntry::new(
            // 1 hour extra
            datetime(period_start, 8, 0, 0),
            Some(datetime(period_start, 17, 0, 0)),
        )?,
        TimeEntry::new(
            // Full day
            datetime(period_start + Duration::days(1), 8, 0, 0),
            Some(datetime(period_start + Duration::days(1), 16, 0, 0)),
        )?,
        TimeEntry::new(
            // Full day
            datetime(period_start + Duration::days(2), 8, 0, 0),
            Some(datetime(period_start + Duration::days(2), 16, 0, 0)),
        )?,
    ])?;
    let reqs = mon_fri_requirements();
    let estimate = period.estimate_time_to_work(&timecard, &reqs);
    let day = date(2026, 3, 30);
    assert_dur_opt_eq(estimate[&day], Some(Duration::hours(9)), day);
    let day = date(2026, 3, 31);
    assert_dur_opt_eq(estimate[&day], Some(Duration::hours(8)), day);
    let day = date(2026, 4, 1);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) - Duration::minutes(30)),
        day,
    );
    let day = date(2026, 4, 2);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) - Duration::minutes(15)),
        day,
    );
    let day = date(2026, 4, 3);
    assert_dur_opt_eq(
        estimate[&day],
        Some(Duration::hours(8) - Duration::minutes(15)),
        day,
    );
    Ok(())
}

#[test]
fn estimate_time_to_work_exempt_days_do_not_contribute_to_deficit() {
    todo!()
}

#[test]
fn estimate_time_to_work_duration_less_than_tick_does_not_contribute() {
    todo!()
}
