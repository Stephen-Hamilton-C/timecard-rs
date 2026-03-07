use chrono::{DateTime, Datelike, Local};
use clap::{Args, builder::ValueParser};
use colored::Colorize;
use timecard::{TimeEntry, Timecard};

use crate::format;


#[derive(Args, Debug)]
pub struct LogArgs {
    /// Show entries on or after this date. If an end date is set but this is omitted, defaults to UNIX epoch (January 1, 1970).
    #[arg(short, long, value_parser = ValueParser::new(format::date_from_input))]
    from_date: Option<DateTime<Local>>,

    /// Show entries on or before this date. If a start date is set but this is omitted, defaults to today.
    #[arg(short, long, value_parser = ValueParser::new(format::date_from_input))]
    to_date: Option<DateTime<Local>>,

    /// Show entries from this specific date. If a date range is also provided, range entries are shown first, followed by entries from this date.
    #[arg(value_parser = ValueParser::new(format::date_from_input))]
    date: Option<DateTime<Local>>,

    /// Show every stored entry, ignoring any date range or specific day filters.
    #[arg(short, long)]
    all: bool,

    /// Prints entries in CSV format, using `,` as a separator. A custom separator can be provided to override the default.
    #[arg(long, num_args = 0..=1, default_missing_value = ",")]
    csv: Option<String>,
}

enum OutputFormat {
    Pretty,
    PrettyByDay,
    Csv(String),
}

impl LogArgs {
    pub fn new(day: Option<DateTime<Local>>, all: bool) -> LogArgs {
        LogArgs {
            from_date: None,
            to_date: None,
            date: day,
            all,
            csv: None,
        }
    }
}

fn get_entry_csv(entry: &TimeEntry, delimiter: &str) -> String {
    let start = format::datetime(&entry.start());
    let end = entry.end().map_or("".into(), |e| format::datetime(&e));
    format!("{}{}{}", start, delimiter, end)
}

fn get_entry_log(entry: &TimeEntry) -> String {
    let today = Local::now().num_days_from_ce();
    let start_str = if entry.start().num_days_from_ce() != today && (entry.end().is_none() || entry.end().is_some_and(|end| end.num_days_from_ce() == today)) {
        format::datetime(&entry.start())
    } else {
        format::time(&entry.start())
    };
    let mut log = format!("{}: {}", "IN".green(), start_str);
    if let Some(end) = entry.end() {
        log += &format!("\t{}: {}", "OUT".red(), format::time(&end));
    }

    log
}

fn log_for_entries(entries: &[&TimeEntry], format: OutputFormat) {
    match format {
        OutputFormat::Pretty => {
            // TODO: Print summary for each day
            for entry in entries {
                println!("{}", get_entry_log(entry));
            }
        }
        OutputFormat::PrettyByDay => {
            let mut current_day: Option<DateTime<Local>> = None;
            for entry in entries {
                if current_day.is_none() || current_day.is_some_and(|day| day.num_days_from_ce() != entry.start().num_days_from_ce()) {
                    if current_day.is_some() {
                        println!();
                    }
                    current_day = Some(entry.start().clone());
                    println!("{}:", format::date(&entry.start()).cyan())
                }

                println!("  {}", get_entry_log(entry));
            }
        }
        OutputFormat::Csv(delimiter) => {
            println!("Start{}End", delimiter);
            for entry in entries {
                println!("{}", get_entry_csv(entry, &delimiter));
            }
        }
    }
}

pub fn log(args: &LogArgs, timecard: &Timecard) -> anyhow::Result<()> {
    let show_range = args.from_date.is_some() || args.to_date.is_some();
    let show_day = (args.from_date.is_none() && args.to_date.is_none()) || args.date.is_some();

    if args.all {
        if timecard.entries().is_empty() {
            eprintln!("{}", "No logs exist.".yellow());
            return Ok(())
        }

        let entries: Vec<&TimeEntry> = timecard.entries().iter().collect();
        if let Some(csv_sep) = &args.csv {
            log_for_entries(&entries, OutputFormat::Csv(csv_sep.into()));
        } else {
            println!("{}", "All logged entries:".bold());
            log_for_entries(&entries, OutputFormat::PrettyByDay);
        }
        return Ok(())
    }

    if show_range {
        let from_date = args.from_date.unwrap_or(DateTime::UNIX_EPOCH.with_timezone(&Local));
        let to_date = args.to_date.unwrap_or(Local::now());
        let entries = timecard.filter_by_date_range(&from_date, &to_date);

        if entries.is_empty() && !show_day {
            eprintln!("{}", "No entries fall within the given date range".yellow());
        } else if !entries.is_empty() {
            if let Some(csv_sep) = &args.csv {
                log_for_entries(&entries, OutputFormat::Csv(csv_sep.into()));
            } else {
                if args.from_date.is_some() && args.to_date.is_some() {
                    println!("Entries from {} to {}:", format::date(&from_date).cyan(), format::date(&to_date).cyan());
                } else if args.from_date.is_some() {
                    println!("Entries from {} to {}:", format::date(&from_date).cyan(), "today".cyan().italic());
                } else if args.to_date.is_some() {
                    println!("Entries from {} to {}:", "forever ago".cyan().italic(), format::date(&to_date).cyan());
                }
                log_for_entries(&entries, OutputFormat::Pretty);
            }
        }
    }

    if show_day {
        let day = args.date.unwrap_or(Local::now());
        let entries = timecard.filter_by_day(&day);

        if entries.is_empty() && !show_range {
            eprintln!("{} {}", "No entries exist for".yellow(), format::date(&day).yellow());
        } else if !entries.is_empty() {
            if let Some(csv_sep) = &args.csv {
                log_for_entries(&entries, OutputFormat::Csv(csv_sep.into()));
            } else {
                if args.date.is_none() {
                    println!("Entries for {}:", "today".cyan().italic());
                } else {
                    println!("Entries for {}:", format::date(&day).cyan());
                }
                log_for_entries(&entries, OutputFormat::Pretty);
            }
        }
    }

    Ok(())
}
