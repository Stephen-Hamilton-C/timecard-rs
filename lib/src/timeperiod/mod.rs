use core::fmt;
use std::{collections::HashMap, hash::Hash};

use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{Timecard, error::ValidationError, serializable::SerializableWeekdaySet};

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WorkPeriodRequirements {
    pub work_days_of_week: SerializableWeekdaySet,
    pub work_day_duration: Duration,
    pub exempt_days: Vec<NaiveDate>,
    pub round_durations_to: Duration,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TimePeriod {
    start: NaiveDate,
    end: NaiveDate,

    #[serde(skip_serializing)]
    today_override: Option<NaiveDate>,
}

impl TimePeriod {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Result<TimePeriod, ValidationError> {
        let period = TimePeriod {
            start,
            end,
            today_override: None,
        };

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

    fn today(&self) -> i32 {
        self.today_override
            .unwrap_or(Utc::now().date_naive())
            .num_days_from_ce()
    }

    fn get_time_already_worked(
        &self,
        work_days: &Vec<NaiveDate>,
        timecard: Option<&Timecard>,
        requirements: &WorkPeriodRequirements,
    ) -> HashMap<NaiveDate, Duration> {
        let today = self.today();
        if self.start.num_days_from_ce() > today {
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
            if let Some(timecard) = timecard {
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
        let today = self.today();
        if self.end.num_days_from_ce() <= today {
            return HashMap::new();
        }

        let mut map: HashMap<NaiveDate, Duration> = HashMap::new();

        let mut duration_needed = extra_duration_needed;
        let future_work_days: Vec<_> = work_days
            .iter()
            .filter(|work_day| work_day.num_days_from_ce() >= today)
            .collect();

        for future_work_day in future_work_days.clone() {
            map.insert(*future_work_day, requirements.work_day_duration);
        }

        let direction = if duration_needed > Duration::zero() {
            1
        } else if duration_needed < Duration::zero() {
            duration_needed = -duration_needed;
            -1
        } else {
            0
        };
        while duration_needed > Duration::zero() {
            for future_work_day in future_work_days.clone() {
                map.entry(*future_work_day)
                    .and_modify(|d| *d += requirements.round_durations_to * direction)
                    .or_insert(requirements.work_day_duration - requirements.round_durations_to);
                duration_needed -= requirements.round_durations_to;
                if duration_needed <= Duration::zero() {
                    break;
                }
            }
        }

        map
    }

    pub fn get_dates(&self) -> Vec<NaiveDate> {
        let mut all_days = vec![];
        let mut current_day = self.start;
        loop {
            all_days.push(current_day);
            current_day += Duration::days(1);

            if current_day > self.end {
                break;
            }
        }
        all_days
    }

    pub fn estimate_time_to_work(
        &self,
        timecard: &Timecard,
        requirements: &WorkPeriodRequirements,
    ) -> HashMap<NaiveDate, Option<Duration>> {
        let all_days = self.get_dates();
        let work_days = all_days
            .iter()
            .filter(|day| {
                requirements.work_days_of_week.0.contains(day.weekday())
                    && !requirements.exempt_days.contains(day)
            })
            .cloned()
            .collect();

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

        let mut duration_map = map_to_option(past_duration_map);
        duration_map.extend(map_to_option(future_duration_map));
        for day in all_days {
            if !duration_map.contains_key(&day) {
                duration_map.insert(day, None);
            }
        }
        duration_map
    }
}

impl fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

fn map_to_option<K: Eq + Hash, V>(map: HashMap<K, V>) -> HashMap<K, Option<V>> {
    map.into_iter().map(|(k, v)| (k, Some(v))).collect()
}
