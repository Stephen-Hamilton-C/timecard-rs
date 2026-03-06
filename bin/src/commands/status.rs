use chrono::{Duration, Local};
use colored::Colorize;
use timecard::Timecard;

use crate::{AppPaths, format, traits::TimecardFile};

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
        println!("Clocked {} since {}", "IN".green(), format::time(&last_entry.start).green());
    } else {
        let last_entry = entries.last().unwrap();
        println!("Clocked {} since {}", "OUT".red(), format::time(&last_entry.end.unwrap()).red());
    }

    let duration_worked = timecard.get_duration_worked(&now, true);
    let duration_on_break = timecard.get_duration_on_break(&now, true);
    // TODO: get expected work duration from config
    let end_time = timecard.get_expected_end_time(Duration::hours(8), &now).unwrap();
    println!("Worked for {}", format::duration(&duration_worked));
    println!("On break for {}", format::duration(&duration_on_break));
    println!("Expected end time: {}", format::time(&end_time));
}
