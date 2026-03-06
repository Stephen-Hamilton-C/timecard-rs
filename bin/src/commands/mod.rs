mod status;
mod clock_in;
mod clock_out;
mod clean;
mod log;
mod undo;

use clap::Subcommand;

pub use status::status;
pub use clock_in::{clock_in, InArgs};
pub use clock_out::{clock_out, OutArgs};
pub use clean::{clean, CleanArgs};
pub use log::{log, LogArgs};
pub use undo::{undo, UndoArgs};


#[derive(Subcommand)]
pub enum Commands {
    /// Shows current clocked status
    Status,
    In(InArgs),
    Out(OutArgs),
    Clean(CleanArgs),
    Log(LogArgs),
    Undo(UndoArgs),
}
