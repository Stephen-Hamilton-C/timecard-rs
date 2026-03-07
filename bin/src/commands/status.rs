use chrono::Local;
use colored::Colorize;
use timecard::Timecard;

use crate::{format, config::Config};

pub fn status(timecard: &Timecard) {
    let config = Config::get();
    let now = Local::now();
    let entries = timecard.filter_by_day(&now);

    if entries.is_empty() {
        println!("{}", "No log for today".yellow());
        return
    } else if timecard.is_clocked_in() {
        let last_entry = entries.last().unwrap();
        println!("Clocked {} since {}", "IN".green(), format::time(&last_entry.start).green());
    } else {
        let last_entry = entries.last().unwrap();
        println!("Clocked {} since {}", "OUT".red(), format::time(&last_entry.end.unwrap()).red());
    }

    let duration_worked = timecard.get_duration_worked(&now, true);
    let duration_on_break = timecard.get_duration_on_break(&now, true);
    let end_time = timecard.get_expected_end_time(config.work_duration, &now).unwrap();
    println!("Worked for {}", format::duration(&duration_worked).green());
    println!("On break for {}", format::duration(&duration_on_break).red());
    println!("Expected end time: {}", format::time(&end_time).cyan());
}
