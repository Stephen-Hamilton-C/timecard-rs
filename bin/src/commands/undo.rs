use std::path::PathBuf;

use chrono::{DateTime, Local};
use timecard::Timecard;

use crate::{commands::{self, LogArgs}, traits::Saveable};


pub fn undo(timecard: &mut Timecard, timecard_path: &PathBuf) {
    let mut undo_day: Option<DateTime<Local>> = None;
    if let Some(last_time) = timecard.entries().last() {
        undo_day = Some(last_time.end.unwrap_or(last_time.start));
    }

    // TODO: expect
    timecard.undo().expect("Failed to undo");
    // TODO: expect
    timecard.save(timecard_path).expect("Failed to save Timecard");

    println!("Successfully removed last entry.");
    commands::log(
        &LogArgs::new(undo_day, undo_day.is_none()),
        timecard,
    );
}
