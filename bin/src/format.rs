use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use colored::Colorize;
use timecard::Timecard;

use crate::config::Config;


pub fn time(datetime: &DateTime<Utc>) -> String {
    let config = Config::get();
    let fmt = &config.time_fmt;
    datetime.with_timezone(&Local).format(fmt).to_string()
}

pub fn date(datetime: &DateTime<Utc>) -> String {
    let config = Config::get();
    let fmt = &config.date_fmt;
    datetime.with_timezone(&Local).format(fmt).to_string()
}

pub fn datetime(datetime: &DateTime<Utc>) -> String {
    let config = Config::get();
    let fmt = &config.datetime_fmt;
    datetime.with_timezone(&Local).format(fmt).to_string()
}

pub fn time_or_datetime(date_time: &DateTime<Utc>, now: &DateTime<Utc>) -> String {
    if date_time.num_days_from_ce() == now.num_days_from_ce() {
        time(&date_time)
    } else {
        datetime(&date_time)
    }
}

pub fn duration(duration: &Duration) -> String {
    let config = Config::get();
    let fmt = &config.duration_fmt;

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;
    let seconds = duration.num_seconds();
    let hours_rounded = (seconds as f64 / 3600.0).round();
    let hours_tenths = ((seconds as f64 / 360.0).round() * 6.0) / 60.0;
    let hours_quarters = ((seconds as f64 / 900.0).round() * 15.0) / 60.0;
    let hours_halves = ((seconds as f64 / 1800.0).round() * 30.0) / 60.0;
    fmt.replace("%ht", &hours_tenths.to_string())
        .replace("%hq", &hours_quarters.to_string())
        .replace("%hh", &hours_halves.to_string())
        .replace("%HH", &hours.to_string())
        .replace("%HR", &hours_rounded.to_string())
        .replace("%MM", &minutes.to_string())
        .replace(">>H", &format!("{:02}", hours))
        .replace(">>M", &format!("{:02}", minutes))
}

fn from_input(input: &str) -> Option<DateTime<Utc>> {
    if let Ok(std_dur) = humantime::parse_duration(input) {
        if let Ok(td) = Duration::from_std(std_dur) {
            return Some(Utc::now() - td)
        }
    }

    if let Ok(std_time) = humantime::parse_rfc3339_weak(input) {
        let local_time: DateTime<Local> = std_time.into();
        return Some(local_time.to_utc())
    }

    None
}

pub fn time_from_input(input: &str) -> Result<DateTime<Utc>, String> {
    if let Some(dt) = from_input(input) {
        return Ok(dt)
    }

    const TIME_FORMATS: &[&str] = &["%H:%M", "%H:%M:%S", "%I:%M%p", "%I:%M %p", "%I:%M:%S%p", "%I:%M:%S %p", "%I%p", "%I %p", "%I%M%p", "%I%M%S%p"];
    let time = TIME_FORMATS.iter().find_map(|fmt| NaiveTime::parse_from_str(input, fmt).ok());
    if let Some(specific_time) = time {
        let naive_dt = Local::now().date_naive().and_time(specific_time);
        return naive_dt.and_local_timezone(Local).single().map(|dt| dt.to_utc()).ok_or("Failed to parse NaiveTime".into())
    }

    const DATE_PREFIXES: &[&str] = &["%Y-%m-%dT", "%Y-%m-%d ", "%Y%m%d_", "%Y%m%dT"];
    let datetime_formats: Vec<String> = DATE_PREFIXES.iter()
        .flat_map(|date| TIME_FORMATS.iter().map(move |time| format!("{}{}", date, time)))
        .collect();
    let datetime = datetime_formats.iter().find_map(|fmt| NaiveDateTime::parse_from_str(input, fmt).ok());
    if let Some(specific_datetime) = datetime {
        return specific_datetime.and_local_timezone(Local).single().map(|dt| dt.to_utc()).ok_or("Failed to parse NaiveDateTime".into())
    }

    if let Ok(minutes) = input.parse::<i64>() {
        return Ok(Utc::now() - Duration::minutes(minutes))
    }

    if input.to_lowercase() == "now" {
        return Ok(Utc::now())
    }

    Err("Invalid datetime format".into())
}

pub fn date_from_input(input: &str) -> Result<DateTime<Utc>, String> {
    if let Some(dt) = from_input(input) {
        return Ok(dt)
    }

    const DATE_FORMATS: &[&str] = &["%Y-%m-%d", "%m/%d/%Y"];
    let date = DATE_FORMATS.iter().find_map(|fmt| NaiveDate::parse_from_str(input, fmt).ok());
    if let Some(specific_date) = date {
        let naive_dt = specific_date.and_time(NaiveTime::parse_from_str("12:00", "%H:%M").unwrap());
        return Local.from_local_datetime(&naive_dt).single().map(|dt| dt.to_utc()).ok_or("Failed to parse NaiveDate".into())
    }

    if let Ok(days) = input.parse::<i64>() {
        return Ok(Utc::now() - Duration::days(days))
    }

    if input.to_lowercase() == "today" {
        return Ok(Utc::now())
    }

    Err("Invalid datetime format".into())
}


pub fn print_status(timecard: &Timecard, date: &DateTime<Utc>) {
    let config = Config::get();
    let duration_worked = timecard.get_duration_worked(&date, true);
    let duration_on_break = timecard.get_duration_on_break(&date, true);
    let end_time = timecard.get_expected_end_time(config.work_duration, &date).unwrap();
    println!("Worked for {}", duration(&duration_worked).green());
    println!("On break for {}", duration(&duration_on_break).red());
    println!("Expected end time: {}", time(&end_time).cyan());
}