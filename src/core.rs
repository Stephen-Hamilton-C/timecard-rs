use chrono::naive::NaiveDateTime;
use json::object;
use std::process;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct TimeEntry {
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
}

impl TimeEntry {
    pub fn serialize(&self) -> String {
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

        data.dump()
    }

    pub fn from(json: String) -> TimeEntry {
        let data = json::parse(&json).unwrap_or_else(|_| {
			deserialize_error("Data parse error!");
        });

        if data["start"].is_null() {
            deserialize_error("Start time cannot be null!");
        }

		let start_data = data["start"].as_i64().unwrap_or_else(|| {
			deserialize_error("Start time isn't a number!");
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

fn deserialize_error(msg: &str) -> ! {
    eprintln!("{msg} Is your timecard file corrupt?");
    process::exit(1);
}
