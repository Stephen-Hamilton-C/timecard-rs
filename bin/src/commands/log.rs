use chrono::{DateTime, Datelike, Local};
use clap::{Args, builder::ValueParser};
use colored::Colorize;
use timecard::{TimeEntry, Timecard};

use crate::format;


#[derive(Args, Debug)]
pub struct LogArgs {
    #[arg(short, long, value_parser = ValueParser::new(format::date_from_input))]
    pub from_date: Option<DateTime<Local>>,

    #[arg(short, long, value_parser = ValueParser::new(format::date_from_input))]
    pub to_date: Option<DateTime<Local>>,

    #[arg(value_parser = ValueParser::new(format::date_from_input))]
    pub day: Option<DateTime<Local>>,

    #[arg(short, long)]
    pub all: bool,
}

fn get_entry_log(entry: &TimeEntry) -> String {
    let today = Local::now().num_days_from_ce();
    let start_str = if entry.start.num_days_from_ce() != today && (entry.end.is_none() || entry.end.is_some_and(|end| end.num_days_from_ce() == today)) {
        format::datetime(&entry.start)
    } else {
        format::time(&entry.start)
    };
    let mut log = format!("{}: {}", "IN".green(), start_str);
    if let Some(end) = entry.end {
        log += &format!("\t{}: {}", "OUT".red(), format::time(&end));
    }

    log
}

fn log_for_entries(entries: &[&TimeEntry]) {
    // TODO: Print summary for each day
    for entry in entries {
        println!("{}", get_entry_log(entry));
    }
}

pub fn log(args: &LogArgs, timecard: &Timecard) {
    let show_range = args.from_date.is_some() || args.to_date.is_some();
    let show_day = (args.from_date.is_none() && args.to_date.is_none()) || args.day.is_some();

    if args.all {
        if timecard.entries().is_empty() {
            println!("{}", "No logs exist.".yellow());
            return
        }

        println!("{}", "All logged entries:".bold());
        let mut current_day: Option<DateTime<Local>> = None;
        for entry in timecard.entries() {
            if current_day.is_none() || current_day.is_some_and(|day| day.num_days_from_ce() != entry.start.num_days_from_ce()) {
                if current_day.is_some() {
                    println!();
                }
                current_day = Some(entry.start.clone());
                println!("{}:", format::date(&entry.start).cyan())
            }

            println!("  {}", get_entry_log(entry));
        }
        return
    }

    if show_range {
        let from_date = args.from_date.unwrap_or(DateTime::UNIX_EPOCH.with_timezone(&Local));
        let to_date = args.to_date.unwrap_or(Local::now());
        let entries = timecard.filter_by_date_range(&from_date, &to_date);

        if entries.is_empty() && !show_day {
            println!("{}", "No entries fall within the given date range".yellow());
        } else if !entries.is_empty() {
            if args.from_date.is_some() && args.to_date.is_some() {
                println!("Entries from {} to {}:", format::date(&from_date).cyan(), format::date(&to_date).cyan());
            } else if args.from_date.is_some() {
                println!("Entries from {} to {}:", format::date(&from_date).cyan(), "today".cyan().italic());
            } else if args.to_date.is_some() {
                println!("Entries from {} to {}:", "forever ago".cyan().italic(), format::date(&to_date).cyan());
            }
            log_for_entries(&entries);
        }
    }

    if show_day {
        let day = args.day.unwrap_or(Local::now());
        let entries = timecard.filter_by_day(&day);

        if entries.is_empty() && !show_range {
            println!("{} {}", "No entries exist for".yellow(), format::date(&day).yellow());
        } else if !entries.is_empty() {
            if args.day.is_none() {
                println!("Entries for {}:", "today".cyan().italic());
            } else {
                println!("Entries for {}:", format::date(&day).cyan());
            }
            log_for_entries(&entries);
        }
    }
}
