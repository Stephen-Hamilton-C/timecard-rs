use core::fmt;
use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, Utc, WeekdaySet};
use serde::{Deserialize, Serialize};

use crate::{Timecard, error::ValidationError};

#[cfg(test)]
mod tests;

// TODO: Need to (de)serialize WeekdaySet
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WorkPeriodRequirements {
    pub work_days_of_week: WeekdaySet,
    pub work_day_duration: Duration,
    pub exempt_days: Vec<NaiveDate>,
    pub round_durations_to: Duration,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TimePeriod {
    start: NaiveDate,
    end: NaiveDate,
}

impl TimePeriod {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Result<TimePeriod, ValidationError> {
        let period = TimePeriod { start, end };

        period.validate().and(Ok(period))
    }

    pub fn start(&self) -> NaiveDate {
        self.start
    }

    pub fn end(&self) -> NaiveDate {
        self.end
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.start > self.end {
            return Err(ValidationError::InvertedTime);
        }

        Ok(())
    }

    fn get_time_already_worked(
        &self,
        work_days: &Vec<NaiveDate>,
        timecard: Option<&Timecard>,
        requirements: &WorkPeriodRequirements,
    ) -> HashMap<NaiveDate, Duration> {
        let today = Utc::now().num_days_from_ce();
        if self.start.num_days_from_ce() >= today {
            return HashMap::new();
        }

        // Period overlaps the past, need to determine how much time has already been worked
        let mut map = HashMap::new();
        let past_work_days: Vec<_> = work_days
            .iter()
            .filter(|work_day| work_day.num_days_from_ce() < today)
            .collect();
        for past_work_day in past_work_days {
            let duration_worked_this_day: Duration;
            if requirements.exempt_days.contains(past_work_day) {
                duration_worked_this_day = requirements.work_day_duration
            } else if let Some(timecard) = timecard {
                duration_worked_this_day = timecard.get_duration_worked(past_work_day, true);
            } else {
                // No timecard, must be calculating expected time already worked
                duration_worked_this_day = requirements.work_day_duration;
            }
            map.insert(*past_work_day, duration_worked_this_day);
        }

        map
    }

    fn get_expected_future_durations(
        &self,
        work_days: &Vec<NaiveDate>,
        extra_duration_needed: Duration,
        requirements: &WorkPeriodRequirements,
    ) -> HashMap<NaiveDate, Duration> {
        let today = Utc::now().num_days_from_ce();
        if self.end.num_days_from_ce() < today {
            // TODO: Return type?
            return HashMap::new();
        }

        let mut map: HashMap<NaiveDate, Duration> = HashMap::new();

        let mut duration_needed = extra_duration_needed;
        let future_work_days: Vec<_> = work_days
            .iter()
            .filter(|work_day| work_day.num_days_from_ce() >= today)
            .filter(|future_work_day| {
                return if requirements.exempt_days.contains(*future_work_day) {
                    duration_needed -= requirements.work_day_duration;
                    false
                } else {
                    true
                };
            })
            .collect();
        while duration_needed > Duration::zero() {
            for future_work_day in future_work_days.clone() {
                map.entry(*future_work_day)
                    .and_modify(|d| *d -= requirements.round_durations_to)
                    .or_insert(requirements.work_day_duration - requirements.round_durations_to);
                duration_needed -= requirements.round_durations_to;
                if duration_needed <= Duration::zero() {
                    break;
                }
            }
        }

        map
    }

    pub fn estimate_time_to_work(
        &self,
        timecard: &Timecard,
        requirements: &WorkPeriodRequirements,
    ) -> HashMap<NaiveDate, Duration> {
        let mut work_days = vec![self.start];
        let mut current_day = self.start;
        while current_day != self.end {
            current_day += Duration::days(1);
            if requirements
                .work_days_of_week
                .contains(current_day.weekday())
            {
                work_days.push(current_day);
            }
        }

        // I need to determine how much time has already been worked
        // Look at past days and get duration worked for each of them
        // Expected work duration = (set of past days - set of exempt_days).count * work_day_duration
        // Subtract the sum of all the past durations from the expected work duration
        let past_duration_map =
            self.get_time_already_worked(&work_days, Some(timecard), requirements);
        let time_worked: Duration = past_duration_map.values().copied().sum();
        let expected_time_worked: Duration = self
            .get_time_already_worked(&work_days, None, requirements)
            .values()
            .copied()
            .sum();
        let delta_worked = expected_time_worked - time_worked;

        // This duration must now be distributed to all future days in the period
        let future_duration_map =
            self.get_expected_future_durations(&work_days, delta_worked, requirements);

        let mut all_durations = past_duration_map;
        all_durations.extend(future_duration_map);
        all_durations
    }
}

impl fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}
