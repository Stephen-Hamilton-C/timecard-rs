mod commands;
mod config;
mod format;
mod traits;
mod chrono_humantime;

use std::fs;

use chrono::{Duration, Local};
use clap::Parser;
use platform_dirs::AppDirs;
use timecard::Timecard;

use crate::{commands::Commands, config::Config, traits::{Loadable, Saveable}};


#[derive(Parser)]
#[command(name = "timecard")]
#[command(
    version,
    propagate_version = true,
    about,
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let app_dirs = AppDirs::new(Some("timecard"), false)
        .ok_or("Failed to determine config file path")?;
    fs::create_dir_all(&app_dirs.config_dir)?;
    fs::create_dir_all(&app_dirs.data_dir)?;
    let config_path = app_dirs.config_dir.join("config.toml");
    let timecard_path = app_dirs.data_dir.join("timecard.json");

    // TODO: expect
    let config = Config::load(&config_path).expect("Failed to load config");
    // TODO: expect
    let mut timecard = Timecard::load(&timecard_path).expect("Failed to load Timecard");

    match &cli.command {
        Some(Commands::Status) | None => commands::status(&timecard),
        Some(Commands::In (args)) => commands::clock_in(args, &mut timecard, &timecard_path),
        Some(Commands::Out (args)) => commands::clock_out(args, &mut timecard, &timecard_path),
        Some(Commands::Log (args)) => commands::log(args, &timecard),
        Some(Commands::Undo) => commands::undo(&mut timecard, &timecard_path),
    }

    if let Some(entry_lifetime_days) = config.entry_lifetime_days {
        let now = Local::now();
        let start_datetime = now - Duration::days(entry_lifetime_days);
        let clean_entries = timecard.filter_by_date_range(&start_datetime, &now);
        let owned_entries = clean_entries.into_iter().cloned().collect();
        let clean_timecard = Timecard::new(owned_entries).unwrap();
        // TODO: expect
        clean_timecard.save(&timecard_path).expect("Failed to save cleaned Timecard");
    }

    Ok(())
}
