use chrono::naive::NaiveDateTime;
use json::object;
use json::JsonValue;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process;

fn data_error(msg: &str) -> ! {
	eprintln!("{msg} Is your timecard file corrupt?");
	process::exit(1);
}

#[cfg(test)]
mod test;

#[derive(Debug)]
#[derive(Clone)]
pub struct TimeEntry {
	pub start: NaiveDateTime,
	pub end: Option<NaiveDateTime>,
}

impl TimeEntry {
}

impl PartialEq for TimeEntry {
	fn eq(&self, other: &Self) -> bool {
		let end_matches = match self.end {
			Some(end) => {
				match other.end {
					Some(other_end) => {
						end.timestamp() == other_end.timestamp()
					}
					None => {
						false
					}
				}
			}
			None => {
				other.end.is_none()
			}
		};
		self.start.timestamp() == other.start.timestamp() && end_matches
	}
}

impl Into<JsonValue> for TimeEntry {
	fn into(self) -> JsonValue {
		let start_timestamp = self.start.timestamp();
		let end_timestamp: Option<i64> = match self.end {
			Some(datetime) => Some(datetime.timestamp()),
			None => None,
		};

		let data = object! {
			start: start_timestamp,
			end: end_timestamp,
		};

		println!("{:#}", data);

		data
	}
}

impl From<&JsonValue> for TimeEntry {
	fn from(data: &JsonValue) -> Self {
		if data["start"].is_null() {
			data_error("Start time cannot be null!");
		}

		let start_data = data["start"].as_i64().unwrap_or_else(|| {
			data_error("Start time isn't a number!");
		});

		let end_data = data["end"].as_i64();
		
		let start = NaiveDateTime::from_timestamp(start_data, 0);
		let end = match end_data {
			Some(data) => Some(NaiveDateTime::from_timestamp(data, 0)),
			None => None,
		};

		TimeEntry { start, end }	
	}
}

pub struct TimeEntries {
	pub entries: Vec<TimeEntry>,
}

impl TimeEntries {
	pub fn new() -> TimeEntries {
		TimeEntries { entries: Vec::new() }
	}

	pub fn save(self) -> Result<(), Box<dyn Error>> {
		let timecard_path = crate::timecard_path();
		let mut timecard = File::create(timecard_path)?;
		
		let data: JsonValue = self.into();
		timecard.write_all(data.dump().as_bytes())?;

		Ok(())
	}

	pub fn load() -> TimeEntries {
		let timecard_path = crate::timecard_path();
		let mut timecard = File::create(timecard_path)
			.expect("Error opening timecard. Is your timecard file corrupt?");

		let mut data = String::new();
		timecard.read_to_string(&mut data)
			.expect("Error reading timecard. Is your timecard file corrupt?");
		
		let parsed_data = json::parse(&data)
			.expect("Unable to parse JSON. Is your timecard file corrupt?");

		TimeEntries::from(&parsed_data)
	}
}

impl Into<JsonValue> for TimeEntries {
	fn into(self) -> JsonValue {
		let mut data = object! {
			entries: [],
		};
		for entry in self.entries {
			data["entries"].push(entry)
				.expect("Should be impossible for entries to not be array.");
		}

		data
	}
}

impl From<&JsonValue> for TimeEntries {
	fn from(data: &JsonValue) -> Self {
		if !data["entries"].is_array() {
			data_error("Entries is not an array!");
		}
		let entries_data = data["entries"].members();
		let mut entries: Vec<TimeEntry> = Vec::new();
		for entry in entries_data {
			entries.push(TimeEntry::from(entry));
		}
		
		TimeEntries { entries }
	}
}
