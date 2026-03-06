use chrono::{Duration, Local};
use colored::Colorize;
use timecard::Timecard;

use crate::{AppPaths, traits::TimecardFile};

pub fn status(paths: AppPaths) {
    // TODO: expect
    let timecard = Timecard::load(&paths.timecard).expect("Failed to load Timecard");
    let now = Local::now();
    let entries = timecard.filter_by_day(&now);

    if entries.is_empty() {
        println!("{}", "No log for today".yellow());
        return
    } else if timecard.is_clocked_in() {
        let last_entry = entries.last().unwrap();
        println!("Clocked {} since {}", "IN".green(), last_entry.start.to_string().green());
    } else {
        let last_entry = entries.last().unwrap();
        println!("Clocked {} since {}", "OUT".red(), last_entry.end.unwrap().to_string().red());
    }

    println!("Worked for {}", timecard.get_duration_worked(&now, true));
    println!("On break for {}", timecard.get_duration_on_break(&now, true));
    // TODO: get expected work duration from config
    println!("Expected end time: {}", timecard.get_expected_end_time(Duration::hours(8), &now).unwrap());
}
