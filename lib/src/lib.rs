//! `timecard` is a library that makes it easy to build applications that keep track of time,
//! such as [`timecard-cli`].
//! 
//! This is primarily built as a Rust learning project, while also creating a tool that's
//! personally useful.
//!
//! [`timecard-cli`]: https://crates.io/crates/timecard-cli
pub mod timecard;
/// Result error definitions
pub mod error;

pub use timecard::{
    TimeEntry,
    Timecard,
};

/// The current version of this library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
