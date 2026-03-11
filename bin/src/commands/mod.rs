mod status;
mod clock_in;
mod clock_out;
mod log;
mod undo;
mod notify;

use clap::Subcommand;

pub use status::status;
pub use clock_in::{clock_in, InArgs};
pub use clock_out::{clock_out, OutArgs};
pub use log::{log, LogArgs};
pub use undo::undo;
pub use notify::{notify, NotifyArgs};


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
