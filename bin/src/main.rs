mod commands;
mod config;
mod format;
mod traits;
mod chrono_humantime;

use std::{fs, path::PathBuf};

use clap::Parser;
use platform_dirs::AppDirs;

use crate::commands::Commands;


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

#[derive(Debug)]
struct AppPaths {
    config: PathBuf,
    timecard: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let app_dirs = AppDirs::new(Some("timecard"), false)
        .ok_or("Failed to determine config file path")?;
    fs::create_dir_all(&app_dirs.config_dir)?;
    fs::create_dir_all(&app_dirs.data_dir)?;
    let paths = AppPaths {
        config: app_dirs.config_dir.join("config.toml"),
        timecard: app_dirs.data_dir.join("timecard.json"),
    };

    match &cli.command {
        Some(Commands::Status) | None => commands::status(paths),
        Some(Commands::In (args)) => commands::clock_in(args, paths),
        Some(Commands::Out (args)) => commands::clock_out(args, paths),
        Some(Commands::Clean (args)) => commands::clean(args),
        Some(Commands::Log (args)) => commands::log(args),
        Some(Commands::Undo) => commands::undo(paths),
    }

    Ok(())
}
