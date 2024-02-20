/*!
Provides project-wide diagnostic types that describe more fine-grained error conditions..

 */

use crate::FileId;
use codespan_reporting::diagnostic::Severity;
use std::{
    fmt::Display,
    sync::{OnceLock, RwLock},
};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Types  Diagnostics and Reporter
// ------------------------------------------------------------------------------------------------

///
/// The type of structured diagnostic reports.
///
pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<FileId>;

// ------------------------------------------------------------------------------------------------
// Diagnostic Level
// ------------------------------------------------------------------------------------------------

///
/// This value determines the level of diagnostics to be emitted by **any** reporter.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SeverityFilter {
    Bug,
    Error,
    Warning,
    Note,
    Help,
    None,
}

static DIAGNOSTIC_LEVEL: OnceLock<RwLock<SeverityFilter>> = OnceLock::new();

///
/// Return the current filter value.
///
pub fn get_diagnostic_level_filter() -> SeverityFilter {
    *DIAGNOSTIC_LEVEL
        .get_or_init(|| RwLock::new(SeverityFilter::None))
        .read()
        .unwrap()
}

///
/// Set the value of the current filter level.
///
pub fn set_diagnostic_level_filter(level: SeverityFilter) {
    trace!("set_diagnostic_level_filter({level})");
    *DIAGNOSTIC_LEVEL
        .get_or_init(|| RwLock::new(SeverityFilter::None))
        .write()
        .unwrap() = level;
}

///
/// Returns `true` if the provided `level` is enabled according to the current filter value.
///
pub fn diagnostic_level_enabled(level: Severity) -> bool {
    match get_diagnostic_level_filter() {
        SeverityFilter::Bug => level >= Severity::Bug,
        SeverityFilter::Error => level >= Severity::Error,
        SeverityFilter::Warning => level >= Severity::Warning,
        SeverityFilter::Note => level >= Severity::Note,
        SeverityFilter::Help => level >= Severity::Help,
        SeverityFilter::None => false,
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for SeverityFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SeverityFilter::Bug => "=bugs",
                SeverityFilter::Error => ">=errors",
                SeverityFilter::Warning => ">=warnings",
                SeverityFilter::Note => ">=notes",
                SeverityFilter::Help => ">=help",
                SeverityFilter::None => "none",
            }
        )
    }
}

impl From<Severity> for SeverityFilter {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Bug => SeverityFilter::Bug,
            Severity::Error => SeverityFilter::Error,
            Severity::Warning => SeverityFilter::Warning,
            Severity::Note => SeverityFilter::Note,
            Severity::Help => SeverityFilter::Help,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod codes;
pub use codes::ErrorCode;

pub mod color;
pub use color::UseColor;

pub mod functions;

pub mod reporter;
pub use reporter::{Reporter, StandardStreamReporter};
