use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::{Args, builder::ValueParser};
use colored::Colorize;
use timecard::Timecard;

use crate::{format, traits::Saveable};


#[derive(Args, Debug)]
pub struct OutArgs {
    /// A time in the past at which the entry ends. Defaults to the current time if omitted.
    #[arg(value_parser = ValueParser::new(format::time_from_input))]
    time: Option<DateTime<Local>>,
}

pub fn clock_out(args: &OutArgs, timecard: &mut Timecard, timecard_path: &PathBuf) -> anyhow::Result<()> {
    let time = args.time.unwrap_or(Local::now());
    timecard.clock_out(time)?;
    timecard.save(timecard_path)?;

    println!("Clocked {} at {}", "out".red(), format::time(&time).red());

    Ok(())
}
