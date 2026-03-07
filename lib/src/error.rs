use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimecardError {
    #[error("Timecard validation error: {0}")]
    TimecardValidation(#[from] ValidationError),

    #[error("Timecard clock error: {0}")]
    TimecardClockError(#[from] ClockError),

    #[error("Timecard undo error: {0}")]
    TimecardUndoError(#[from] UndoError),
}

#[derive(Debug, Error)]
pub enum TimecardFromStrError {
    #[error("Timecard validation error: {0}")]
    TimecardValidation(#[from] ValidationError),

    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("End entry required if this is not the last entry")]
    EndTimeRequired,

    #[error("Entries must be in chronological order")]
    Chronological,

    #[error("Entry start time must come before entry end time")]
    InvertedEntry,
}

#[derive(Debug, Error)]
pub enum ClockError {
    #[error("Already clocked {0}")]
    AlreadyInState(ClockState),

    #[error("Time is in the future")]
    TimeInFuture,

    #[error("Time is before last entry")]
    BeforeLastEntry,
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

#[derive(Debug, Error)]
pub enum UndoError {
    #[error("No more entries to undo")]
    EmptyEntries,
}
