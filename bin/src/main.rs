mod commands;

use std::fs;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let app_dirs = AppDirs::new(Some("timecard"), false)
        .ok_or("Failed to determine config file path")?;
    fs::create_dir_all(&app_dirs.config_dir)?;
    let config_path = app_dirs.config_dir.join("config.toml");

    // TODO: Create config with default if it doesn't exist
    // TODO: Load config and pass it into commands

    match &cli.command {
        Some(Commands::Status) => commands::status(),
        Some(Commands::In (args)) => commands::clock_in(args),
        Some(Commands::Out (args)) => commands::clock_out(args),
        Some(Commands::Clean (args)) => commands::clean(args),
        Some(Commands::Log (args)) => commands::log(args),
        Some(Commands::Undo (args)) => commands::undo(args),
        None => commands::status(),
    }

    Ok(())
}
