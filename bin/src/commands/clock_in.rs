use chrono::{DateTime, Local};
use clap::Args;
use colored::Colorize;
use timecard::Timecard;

use crate::{AppPaths, traits::TimecardFile};


#[derive(Args, Debug)]
pub struct InArgs {
    time: Option<DateTime<Local>>
}

pub fn clock_in(args: &InArgs, paths: AppPaths) {
    // TODO: expect
    let mut timecard = Timecard::load(&paths.timecard).expect("Failed to load Timecard");

    let time = args.time.unwrap_or(Local::now());
    // TODO: expect
    timecard.clock_in(time).expect("Failed to clock in");
    // TODO: expect
    timecard.save(&paths.timecard).expect("Failed to save Timecard");

    println!("Clocked {} at {}", "in".green(), time.to_string().green());
}
