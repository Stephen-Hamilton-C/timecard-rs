use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::{Args, builder::ValueParser};
use colored::Colorize;
use timecard::Timecard;

use crate::{format, traits::Saveable};


#[derive(Args, Debug)]
pub struct InArgs {
    #[arg(value_parser = ValueParser::new(format::time_from_input))]
    time: Option<DateTime<Local>>
}

pub fn clock_in(args: &InArgs, timecard: &mut Timecard, timecard_path: &PathBuf) -> anyhow::Result<()> {
    let time = args.time.unwrap_or(Local::now());
    timecard.clock_in(time)?;
    timecard.save(timecard_path)?;

    println!("Clocked {} at {}", "in".green(), format::time(&time).green());

    Ok(())
}
