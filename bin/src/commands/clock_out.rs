use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::{Args, builder::ValueParser};
use colored::Colorize;
use timecard::Timecard;

use crate::{format, traits::Saveable};


#[derive(Args, Debug)]
pub struct OutArgs {
    #[arg(value_parser = ValueParser::new(format::time_from_input))]
    time: Option<DateTime<Local>>,
}

pub fn clock_out(args: &OutArgs, timecard: &mut Timecard, timecard_path: &PathBuf) {
    let time = args.time.unwrap_or(Local::now());
    // TODO: expect
    timecard.clock_out(time).expect("Failed to clock out");
    // TODO: expect
    timecard.save(timecard_path).expect("Failed to save Timecard");

    println!("Clocked {} at {}", "out".red(), format::time(&time).red());
}
