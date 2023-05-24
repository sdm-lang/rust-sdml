/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// An error was signaled by the standard library I/O functions.
    IoError { source: std::io::Error },
    /// An error was signaled by the standard string conversion functions.
    Utf8Error { source: core::str::Utf8Error },
    /// An error was signaled by the standard string conversion functions.
    FromUtf8Error { source: std::string::FromUtf8Error },
    TracingFilterError { source: tracing_subscriber::filter::ParseError },
    TracingSubscriberError { source:  tracing::subscriber::SetGlobalDefaultError },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an Error from the provided source.
#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    Error::IoError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn utf8_error(source: core::str::Utf8Error) -> Error {
    Error::Utf8Error { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn from_utf8_error(source: std::string::FromUtf8Error) -> Error {
    Error::FromUtf8Error { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_filter_error(source: tracing_subscriber::filter::ParseError) -> Error {
    Error::TracingFilterError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_subscriber_error(source: tracing::subscriber::SetGlobalDefaultError) -> Error {
    Error::TracingSubscriberError { source }
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
                Self::IoError { source } => format!("An I/O error occurred; source: {}", source),
                Self::Utf8Error { source } =>
                    format!("A UTF-8 conversion error occurred; source: {}", source),
                Self::FromUtf8Error { source } =>
                    format!("A UTF-8 conversion error occurred; source: {}", source),
                Self::TracingFilterError { source } =>
                    format!("A error occurred parsing a tracing filter; source: {}", source),
                Self::TracingSubscriberError { source } =>
                    format!("A error occurred setting the tracing subscriber; source: {}", source),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        #[allow(unreachable_patterns)]
        match self {
            Error::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        io_error(source)
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(source: core::str::Utf8Error) -> Self {
        utf8_error(source)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(source: std::string::FromUtf8Error) -> Self {
        from_utf8_error(source)
    }
}

impl From<tracing_subscriber::filter::ParseError> for Error {
    fn from(source: tracing_subscriber::filter::ParseError) -> Self {
        tracing_filter_error(source)
    }
}

impl From<tracing::subscriber::SetGlobalDefaultError> for Error {
    fn from(source: tracing::subscriber::SetGlobalDefaultError) -> Self {
        tracing_subscriber_error(source)
    }
}
