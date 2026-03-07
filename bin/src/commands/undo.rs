use std::path::PathBuf;

use chrono::{DateTime, Local};
use timecard::Timecard;

use crate::{commands::{self, LogArgs}, traits::Saveable};


pub fn undo(timecard: &mut Timecard, timecard_path: &PathBuf) -> anyhow::Result<()> {
    let mut undo_day: Option<DateTime<Local>> = None;
    if let Some(last_time) = timecard.entries().last() {
        undo_day = Some(last_time.end().unwrap_or(last_time.start()));
    }

    timecard.undo()?;
    timecard.save(timecard_path)?;

    println!("Successfully removed last entry.");
    commands::log(
        &LogArgs::new(undo_day, undo_day.is_none()),
        timecard,
    )?;

    Ok(())
}
