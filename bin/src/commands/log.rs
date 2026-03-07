use chrono::{DateTime, Local};
use clap::Args;
use colored::Colorize;
use timecard::Timecard;

use crate::{AppPaths, format, traits::Loadable};


#[derive(Args, Debug)]
pub struct LogArgs {
    #[arg(short, long)]
    from_date: Option<DateTime<Local>>,

    #[arg(short, long)]
    to_date: Option<DateTime<Local>>,
}

pub fn log(args: &LogArgs, paths: &AppPaths) {
    let from_date = args.from_date.unwrap_or(DateTime::UNIX_EPOCH.with_timezone(&Local));
    let to_date = args.to_date.unwrap_or(Local::now());

    // TODO: expect
    let timecard = Timecard::load(&paths.timecard).expect("Failed to load Timecard");
    let entries = timecard.filter_by_date_range(&from_date, &to_date);

    if entries.is_empty() {
        println!("{}", "No entries fall within the given date range".yellow());
        return
    }

    // TODO: Categorize by day
    // TODO: Print summary for each day
    for entry in entries {
        print!("{}: {}", "IN".green(), format::time(&entry.start));
        if let Some(end) = entry.end {
            println!("\t{}: {}", "OUT".red(), format::time(&end));
        } else {
            println!();
        }
    }
}
