mod commands;
mod config;
mod format;
mod traits;
mod chrono_humantime;

use std::fs;

use anyhow::{Context, Result};
use chrono::{Duration, Local};
use clap::Parser;
use platform_dirs::AppDirs;
use timecard::Timecard;

use crate::{commands::Commands, config::Config, traits::{Loadable, Saveable}};


#[derive(Parser)]
#[command(
    name = "timecard",
    about = "Helps you keep track of how long you've worked each day",
    version,
    propagate_version = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let app_dirs = AppDirs::new(Some("timecard"), false)
        .context("Failed to determine data paths")?;
    fs::create_dir_all(&app_dirs.config_dir)
        .context(format!("Failed to create config directory: {}", &app_dirs.config_dir.display()))?;
    fs::create_dir_all(&app_dirs.data_dir)
        .context(format!("Failed to create data directory: {}", &app_dirs.data_dir.display()))?;
    let config_path = app_dirs.config_dir.join("timecard-cli.toml");
    let timecard_path = app_dirs.data_dir.join("timecard.json");

    let config = Config::load(&config_path)?;
    let mut timecard = Timecard::load(&timecard_path)?;

    match &cli.command {
        Some(Commands::Status) | None => commands::status(&timecard)?,
        Some(Commands::In (args)) => commands::clock_in(args, &mut timecard, &timecard_path)?,
        Some(Commands::Out (args)) => commands::clock_out(args, &mut timecard, &timecard_path)?,
        Some(Commands::Log (args)) => commands::log(args, &timecard)?,
        Some(Commands::Undo) => commands::undo(&mut timecard, &timecard_path)?,
    }

    if let Some(entry_lifetime_days) = config.entry_lifetime_days {
        let now = Local::now();
        let start_datetime = now - Duration::days(entry_lifetime_days);
        let clean_entries = timecard.filter_by_date_range(&start_datetime, &now);
        let owned_entries = clean_entries.into_iter().cloned().collect();
        let clean_timecard = Timecard::new(owned_entries).unwrap();
        clean_timecard.save(&timecard_path)
            .context("Failed to save Timecard while housekeeping")?;
    }

    Ok(())
}
