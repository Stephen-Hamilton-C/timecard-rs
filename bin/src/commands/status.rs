use chrono::Utc;
use colored::Colorize;
use timecard::Timecard;

use crate::{format, config::Config};

pub fn status(timecard: &Timecard) -> anyhow::Result<()> {
    let config = Config::get();
    let now = Utc::now();
    let entries = timecard.filter_by_day(&now);

    if entries.is_empty() {
        println!("{}", "No log for today".yellow());
        return Ok(())
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

    let duration_worked = timecard.get_duration_worked(&now, true);
    let duration_on_break = timecard.get_duration_on_break(&now, true);
    let end_time = timecard.get_expected_end_time(config.work_duration, &now).unwrap();
    println!("Worked for {}", format::duration(&duration_worked).green());
    println!("On break for {}", format::duration(&duration_on_break).red());
    println!("Expected end time: {}", format::time(&end_time).cyan());

    Ok(())
}
