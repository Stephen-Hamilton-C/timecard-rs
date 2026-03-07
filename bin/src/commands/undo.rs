use chrono::{DateTime, Local};
use timecard::Timecard;

use crate::{AppPaths, commands::{self, LogArgs}, traits::{Loadable, Saveable}};


pub fn undo(paths: &AppPaths) {
    // TODO: expect
    let mut timecard = Timecard::load(&paths.timecard).expect("Failed to load timecard");

    let mut undo_day: Option<DateTime<Local>> = None;
    if let Some(last_time) = timecard.entries().last() {
        undo_day = Some(last_time.end.unwrap_or(last_time.start));
    }

    // TODO: expect
    timecard.undo().expect("Failed to undo");
    // TODO: expect
    timecard.save(&paths.timecard).expect("Failed to save Timecard");

    println!("Successfully removed last entry.");
    commands::log(
        &LogArgs {
            from_date: None,
            to_date: None,
            day: undo_day,
            all: undo_day.is_none(),
        },
        paths,
    );
}
