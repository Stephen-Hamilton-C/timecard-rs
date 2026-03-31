mod clock_in;
mod clock_out;
mod log;
mod notify;
mod status;
mod undo;

use clap::Subcommand;

pub use clock_in::{InArgs, clock_in};
pub use clock_out::{OutArgs, clock_out};
pub use log::{LogArgs, log};
pub use notify::{NotifyArgs, notify};
pub use status::status;
pub use undo::undo;

#[derive(Subcommand)]
pub enum Commands {
    /// Show whether you are currently clocked in or out.
    Status,
    /// Clock in and start a new time entry.
    In(InArgs),
    /// Clock out and end the current time entry.
    Out(OutArgs),
    /// View time entries for a specific day or date range.
    Log(LogArgs),
    /// Revert the most recent clock in or clock out action.
    Undo,
    /// Send notification if expected end time is hit.
    Notify(NotifyArgs),
}
