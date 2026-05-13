use std::fmt::Display;

use chrono::{DateTime, Utc};
use thiserror::Error;

/// A catch-all for any error that could be thrown by `timecard`
#[derive(Debug, Error)]
pub enum TimecardError {
    #[error("Timecard validation error: {0}")]
    TimecardValidation(#[from] ValidationError),

    #[error("Timecard clock error: {0}")]
    TimecardClockError(#[from] ClockError),

    #[error("Timecard undo error: {0}")]
    TimecardUndoError(#[from] UndoError),
}

/// Errors that can be thrown by `Timecard::from_str`
#[derive(Debug, Error)]
pub enum TimecardFromStrError {
    #[error("Timecard validation error: {0}")]
    TimecardValidation(#[from] ValidationError),

    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
}

/// An error that happened while a Timecard or TimeEntry was being constructed
/// 
/// # Variants
/// 
/// - `EndTimeRequired` - Thrown if an end entry is not the last entry
/// - `Chronological` - Thrown if entries are not in chronological order
/// - `InvertedEntry` - Thrown if an entry has an end time that comes before its start time
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("End entry required if this is not the last entry")]
    EndTimeRequired,

    #[error("Entries must be in chronological order")]
    Chronological,

    #[error("Entry start time must come before entry end time")]
    InvertedEntry,
}

/// An error that happened while trying to clock a Timecard in or out.
/// 
/// # Variants
/// 
/// - `AlreadyInState(ClockState)` - Thrown when trying to clock into a state that the Timecard was already in
/// - `TimeInFuture` - Thrown when trying to clock in or out into the future
/// - `BeforeLastEntry` - Thrown when trying to clock in or out before the last recorded time
#[derive(Debug, Error)]
pub enum ClockError {
    #[error("Already clocked {0}")]
    AlreadyInState(ClockState),

    #[error("Time '{0}' is in the future")]
    TimeInFuture(DateTime<Utc>),

    #[error("Time '{0}' is before last entry")]
    BeforeLastEntry(DateTime<Utc>),
}

#[derive(Debug)]
pub enum ClockState {
    In,
    Out,
}

impl Display for ClockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClockState::In => write!(f, "in"),
            ClockState::Out => write!(f, "out"),
        }
    }
}

/// An error that happened while trying to undo the last time record in a Timecard
/// 
/// # Variants
/// 
/// - `EmptyEntries` - Thrown when trying to undo on a Timecard with no entries
#[derive(Debug, Error)]
pub enum UndoError {
    #[error("No more entries to undo")]
    EmptyEntries,
}
