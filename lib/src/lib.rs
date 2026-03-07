pub mod timecard;
pub mod error;

pub use timecard::{
    TimeEntry,
    Timecard,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
