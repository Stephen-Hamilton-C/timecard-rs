use chrono::{DateTime, Duration, Local, NaiveTime};


// TODO: Read from config
pub fn time(datetime: &DateTime<Local>) -> String {
    let fmt = "%H:%M";
    datetime.format(fmt).to_string()
}

pub fn time_secs(datetime: &DateTime<Local>) -> String {
    let fmt = "%H:%M:%S";
    datetime.format(fmt).to_string()
}

pub fn date(datetime: &DateTime<Local>) -> String {
    let fmt = "%Y-%m-%d";
    datetime.format(fmt).to_string()
}

pub fn datetime(datetime: &DateTime<Local>) -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";
    datetime.format(fmt).to_string()
}

pub fn duration(duration: &Duration) -> String {
    let fmt = "%ht hours";

    let seconds = duration.num_seconds();
    let hours = (seconds as f64 / 3600.0).round();
    let hours_tenths = (seconds as f64 / 360.0).round() / 10.0;
    fmt.replace("%ht", &hours_tenths.to_string())
        // .replace("%hq", &hours_quarters.to_string())
        // .replace("%hh", &hours_halves.to_string())
        .replace("%HH", &hours.to_string())
}

pub fn from_input(input: &str) -> Option<DateTime<Local>> {
    if let Ok(std_dur) = humantime::parse_duration(input) {
        if let Ok(td) = Duration::from_std(std_dur) {
            return Some(Local::now() - td)
        }
    }

    if let Ok(std_time) = humantime::parse_rfc3339_weak(input) {
        return Some(std_time.into())
    }

    const TIME_FORMATS: &[&str] = &["%H:%M", "%H:%M:%S", "%I:%M %p", "%I:%M:%S %p", "%I %p"];
    let time = TIME_FORMATS.iter().find_map(|fmt| NaiveTime::parse_from_str(input, fmt).ok());
    if let Some(specific_time) = time {
        let naive_dt = Local::now().date_naive().and_time(specific_time);
        return naive_dt.and_local_timezone(Local).single()
    }

    if let Ok(minutes) = input.parse::<i64>() {
        return Some(Local::now() - Duration::minutes(minutes))
    }

    None
}
