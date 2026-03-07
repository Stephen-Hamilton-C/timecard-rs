use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

use crate::config::Config;


// TODO: Read from config
pub fn time(datetime: &DateTime<Local>) -> String {
    let fmt = "%H:%M";
    datetime.format(fmt).to_string()
}

pub fn date(datetime: &DateTime<Local>) -> String {
    let fmt = "%Y-%m-%d";
    datetime.format(fmt).to_string()
}

pub fn duration(duration: &Duration) -> String {
    let config = Config::get();
    let fmt = &config.duration_format;

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - hours * 60;
    let seconds = duration.num_seconds();
    let hours_rounded = (seconds as f64 / 3600.0).round();
    let hours_tenths = (seconds as f64 / 360.0).round() / 10.0;
    fmt.replace("%ht", &hours_tenths.to_string())
        // .replace("%hq", &hours_quarters.to_string())
        // .replace("%hh", &hours_halves.to_string())
        .replace("%HH", &hours.to_string())
        .replace("%HR", &hours_rounded.to_string())
        .replace("%MM", &minutes.to_string())
        .replace(">>H", &format!("{:02}", hours))
        .replace(">>M", &format!("{:02}", minutes))
}

fn from_input(input: &str) -> Option<DateTime<Local>> {
    if let Ok(std_dur) = humantime::parse_duration(input) {
        if let Ok(td) = Duration::from_std(std_dur) {
            return Some(Local::now() - td)
        }
    }

    if let Ok(std_time) = humantime::parse_rfc3339_weak(input) {
        return Some(std_time.into())
    }

    None
}

pub fn time_from_input(input: &str) -> Result<DateTime<Local>, String> {
    if let Some(dt) = from_input(input) {
        return Ok(dt)
    }

    const TIME_FORMATS: &[&str] = &["%H:%M", "%H:%M:%S", "%I:%M %p", "%I:%M:%S %p", "%I %p"];
    let time = TIME_FORMATS.iter().find_map(|fmt| NaiveTime::parse_from_str(input, fmt).ok());
    if let Some(specific_time) = time {
        let naive_dt = Local::now().date_naive().and_time(specific_time);
        return naive_dt.and_local_timezone(Local).single().ok_or("Failed to parse NaiveTime".into())
    }

    const DATETIME_FORMATS: &[&str] = &["%Y-%m-%dT%H:%M:%S", "%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"];
    let datetime = DATETIME_FORMATS.iter().find_map(|fmt| NaiveDateTime::parse_from_str(input, fmt).ok());
    if let Some(specific_datetime) = datetime {
        return specific_datetime.and_local_timezone(Local).single().ok_or("Failed to parse NaiveDateTime".into())
    }

    if let Ok(minutes) = input.parse::<i64>() {
        return Ok(Local::now() - Duration::minutes(minutes))
    }

    if input.to_lowercase() == "now" {
        return Ok(Local::now())
    }

    Err("Invalid datetime format".into())
}

pub fn date_from_input(input: &str) -> Result<DateTime<Local>, String> {
    if let Some(dt) = from_input(input) {
        return Ok(dt)
    }

    const DATE_FORMATS: &[&str] = &["%Y-%m-%d"];
    let date = DATE_FORMATS.iter().find_map(|fmt| NaiveDate::parse_from_str(input, fmt).ok());
    if let Some(specific_date) = date {
        let naive_dt = specific_date.and_time(NaiveTime::parse_from_str("12:00", "%H:%M").unwrap());
        return Local.from_local_datetime(&naive_dt).single().ok_or("Failed to parse NaiveDate".into())
    }

    if let Ok(days) = input.parse::<i64>() {
        return Ok(Local::now() - Duration::days(days))
    }

    if input.to_lowercase() == "today" {
        return Ok(Local::now())
    }

    Err("Invalid datetime format".into())
}
