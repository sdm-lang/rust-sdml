/*!
Provides project-wide diagnostic types that describe more fine-grained error conditions..

 */

use crate::FileId;
use codespan_reporting::diagnostic::Severity;
use std::fmt::Display;

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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for SeverityFilter {
    fn default() -> Self {
        Self::Error
    }
}

impl Display for SeverityFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Bug => "=bugs",
                Self::Error => ">=errors",
                Self::Warning => ">=warnings",
                Self::Note => ">=notes",
                Self::Help => ">=help",
                Self::None => "none",
            }
        )
    }
}

impl From<Severity> for SeverityFilter {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Bug => Self::Bug,
            Severity::Error => Self::Error,
            Severity::Warning => Self::Warning,
            Severity::Note => Self::Note,
            Severity::Help => Self::Help,
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
