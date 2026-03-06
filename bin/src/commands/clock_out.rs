use chrono::{DateTime, Local};
use clap::Args;
use colored::Colorize;
use timecard::Timecard;

use crate::{AppPaths, format, traits::{Loadable, Saveable}};


#[derive(Args, Debug)]
pub struct OutArgs {
    time: Option<DateTime<Local>>,
}

pub fn clock_out(args: &OutArgs, paths: AppPaths) {
    // TODO: expect
    let mut timecard = Timecard::load(&paths.timecard).expect("Failed to load Timecard");

    let time = args.time.unwrap_or(Local::now());
    // TODO: expect
    timecard.clock_out(time).expect("Failed to clock out");
    // TODO: expect
    timecard.save(&paths.timecard).expect("Failed to save Timecard");

    println!("Clocked {} at {}", "out".red(), format::time(&time).red());
}
