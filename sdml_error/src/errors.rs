/*!
Provides the project-wide Error and Result types as well as helper functions.
 */

use crate::diagnostics::Diagnostic;
use std::fmt::{Debug, Display};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types  Error and Result
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// An error was signaled by the standard library I/O functions.
    IoError {
        source: std::io::Error,
    },
    /// An error was signaled by the standard string conversion functions.
    Utf8Error {
        source: core::str::Utf8Error,
    },
    /// An error was signaled by the standard string conversion functions.
    FromUtf8Error {
        source: std::string::FromUtf8Error,
    },
    /// An error was signaled while parsing a [`Url`]. Note that the methods `from_file_path` and
    /// `from_directory_path` return `()` on error.
    UrlParseError {
        source: Option<url::ParseError>,
    },
    /// An error was signaled while parsing a tracing filter expression.
    TracingFilterError {
        source: tracing_subscriber::filter::ParseError,
    },
    /// An error was signaled while initializing a tracing subscriber..
    TracingSubscriberError {
        source: tracing::subscriber::SetGlobalDefaultError,
    },
    CodespanReportingError {
        source: codespan_reporting::files::Error,
    },
    /// This allows for a complete `Diagnostic` structure to be passed as an Error.
    LanguageValidationError {
        source: Diagnostic,
    },
    GeneratorError {
        message: String,
    },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! report_and_return {
    ($err: expr) => {
        let err = $err;
        error!("{}", err);
        return err;
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IoError { source } => format!("An I/O error occurred; source: {source}"),
                Self::Utf8Error { source } =>
                    format!("A UTF-8 conversion error occurred; source: {source}"),
                Self::FromUtf8Error { source } =>
                    format!("A UTF-8 conversion error occurred; source: {source}"),
                Self::UrlParseError { source } =>
                    if let Some(source) = source {
                        format!("An error occurred parsing a URL; source: {source}")
                    } else {
                        "An error occurred creating a URL from a file path".to_string()
                    },
                Self::TracingFilterError { source } =>
                    format!("A error occurred parsing a tracing filter; source: {source}"),
                Self::TracingSubscriberError { source } =>
                    format!("A error occurred setting the tracing subscriber; source: {source}"),
                Self::CodespanReportingError { source } =>
                    format!("An error occurred formatting codespan reports; source: {source}"),
                Self::LanguageValidationError { source } =>
                    format!("Validation diagnostic: {}", source.message),
                Self::GeneratorError { message } =>
                    format!("An error occurred in a generator: {message}"),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        #[allow(unreachable_patterns)]
        match self {
            Error::IoError { source } => Some(source),
            Error::Utf8Error { source } => Some(source),
            Self::FromUtf8Error { source } => Some(source),
            Self::TracingFilterError { source } => Some(source),
            Self::TracingSubscriberError { source } => Some(source),
            Self::CodespanReportingError { source } => Some(source),
            _ => None,
        }
    }
}

impl<T> From<Error> for Result<T> {
    fn from(value: Error) -> Self {
        Err(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        report_and_return! {
            Error::IoError { source }
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(source: core::str::Utf8Error) -> Self {
        report_and_return! {
            Error::Utf8Error { source }
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        report_and_return! {
            Error::FromUtf8Error { source }
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(source: url::ParseError) -> Self {
        report_and_return! {
            Error::UrlParseError { source: Some(source) }
        }
    }
}

impl From<tracing_subscriber::filter::ParseError> for Error {
    fn from(source: tracing_subscriber::filter::ParseError) -> Self {
        report_and_return! {
            Error::TracingFilterError { source }
        }
    }
}

impl From<tracing::subscriber::SetGlobalDefaultError> for Error {
    fn from(source: tracing::subscriber::SetGlobalDefaultError) -> Self {
        report_and_return! {
            Error::TracingSubscriberError { source }
        }
    }
}

impl From<codespan_reporting::files::Error> for Error {
    fn from(source: codespan_reporting::files::Error) -> Self {
        report_and_return! {
            Error::CodespanReportingError { source }
        }
    }
}

impl From<Diagnostic> for Error {
    fn from(source: Diagnostic) -> Self {
        report_and_return! {
            Self::LanguageValidationError { source }
        }
    }
}
