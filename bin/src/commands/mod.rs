mod status;
mod clock_in;
mod clock_out;
mod log;
mod undo;

use clap::Subcommand;

pub use status::status;
pub use clock_in::{clock_in, InArgs};
pub use clock_out::{clock_out, OutArgs};
pub use log::{log, LogArgs};
pub use undo::undo;


#[derive(Subcommand)]
pub enum Commands {
    /// Shows current clocked status
    Status,
    In(InArgs),
    Out(OutArgs),
    Log(LogArgs),
    Undo,
}
