use chrono::Utc;
use colored::Colorize;
use timecard::Timecard;

use crate::format;

pub fn status(timecard: &Timecard) -> anyhow::Result<()> {
    let now = Utc::now();
    let today = now.date_naive();
    let entries = timecard.filter_by_day(&today);

    if entries.is_empty() {
        println!("{}", "No log for today".yellow());
        return Ok(());
    } else if timecard.is_clocked_in() {
        let last_entry = entries.last().unwrap();
        let fmt_time = format::time_or_datetime(&last_entry.start(), &now);
        println!("Clocked {} since {}", "IN".green(), fmt_time.green());
    } else {
        let last_entry = entries.last().unwrap();
        // We're clocked out, so there must be an end entry
        let fmt_time = format::time_or_datetime(&last_entry.end().unwrap(), &now);
        println!("Clocked {} since {}", "OUT".red(), fmt_time.red());
    }

    format::print_status(timecard, &today);
    Ok(())
}
