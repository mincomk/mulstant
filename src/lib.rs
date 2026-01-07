//! A simple timer utility for recording multiple durations between events.
//!
//! `mulstant` allows you to track intervals between subsequent calls to its `record` method
//! and provides a summary of all recorded intervals along with the total duration.

use std::time::{Duration, Instant};

/// A single recorded interval with a name and duration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Record {
    /// The name of the recorded interval.
    pub name: String,
    /// The duration of the interval.
    pub duration: Duration,
}

/// The result of a `Mulstant` measurement session.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MulstantResult {
    /// All recorded intervals.
    pub records: Vec<Record>,
    /// The total duration from the creation of `Mulstant` to its finalization.
    pub total_duration: Duration,
}

/// A timer that records multiple intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mulstant {
    initial_instant: Instant,
    instant: Instant,
    records: Vec<Record>,
}

impl Mulstant {
    /// Creates a new `Mulstant` instance and starts the timer.
    pub fn new() -> Self {
        Self {
            initial_instant: Instant::now(),
            instant: Instant::now(),
            records: Vec::new(),
        }
    }

    /// Records the duration since the last call to `record` (or since `new`) with the given name.
    pub fn record(&mut self, name: &str) {
        let now = Instant::now();
        let duration = self.instant.elapsed();
        self.records.push(Record {
            name: name.to_string(),
            duration,
        });
        self.instant = now;
    }

    /// Finalizes the timer and returns a `MulstantResult` containing all records and the total duration.
    pub fn finalize(self) -> MulstantResult {
        let total_duration = self.initial_instant.elapsed();
        MulstantResult {
            records: self.records,
            total_duration,
        }
    }
}

impl std::fmt::Debug for MulstantResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_alternate = f.alternate();
        let alternate_space = if is_alternate { "  " } else { "" };
        let alternate_n = if is_alternate { "\n" } else { " " };
        let alternate_comma = if is_alternate { "\n" } else { ", " };
        write!(f, "MulstantResult {{{}", alternate_n)?;
        for record in &self.records {
            write!(
                f,
                "{}{}: {:.2?}{}",
                alternate_space, record.name, record.duration, alternate_comma
            )?;
        }
        write!(
            f,
            "{}Total Duration: {:.2?}{}",
            alternate_space, self.total_duration, alternate_n
        )?;
        write!(f, "}}")
    }
}

impl MulstantResult {
    /// Returns a formatted string summary of the records and total duration.
    pub fn summary(&self) -> String {
        let mut summary = String::new();
        for record in &self.records {
            summary.push_str(&format!("{}: {:.2?}\n", record.name, record.duration));
        }
        summary.push_str(&format!("Total Duration: {:.2?}\n", self.total_duration));
        summary
    }
}
