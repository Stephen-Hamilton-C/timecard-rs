use std::env;

use chrono::{Utc, SubsecRound};
use timecard::core::{TimeEntries, TimeEntry};

fn main() {
	let args: Vec<String> = env::args().collect();
	dbg!(args);
	let entry = TimeEntry { start: Utc::now().naive_local().round_subsecs(0), end: None };
	let entries = TimeEntries { entries: vec![entry] };
	entries.save().expect("Unable to save");
	let entries = TimeEntries::load();
	dbg!(entries);
}
