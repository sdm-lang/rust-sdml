/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use std::fmt::{Debug, Display};
use tracing::error;

// ------------------------------------------------------------------------------------------------
// Public Types
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
    TracingFilterError {
        source: tracing_subscriber::filter::ParseError,
    },
    TracingSubscriberError {
        source: tracing::subscriber::SetGlobalDefaultError,
    },
    InvalidIdentifierError {
        input: String,
    },
    InvalidLanguageTagError {
        input: String,
    },
    InvalidNodeKind {
        rule: String,
        got: String,
    },
    UnexpectedNodeKind {
        rule: String,
        expected: String,
        got: String,
    },
    MissingNodeKind {
        rule: String,
        expected: String,
    },
    InvalidValueForType {
        value: String,
        type_name: String,
    },
    ModuleFileNotFound {
        name: String,
    },
    ModuleParseError {
        rule: Option<String>,
        node_name: String,
        start: usize,
        end: usize,
    },
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
    error!("Wrapping error: {}", source);
    Error::IoError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn utf8_error(source: core::str::Utf8Error) -> Error {
    error!("Wrapping error: {}", source);
    Error::Utf8Error { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn from_utf8_error(source: std::string::FromUtf8Error) -> Error {
    error!("Wrapping error: {}", source);
    Error::FromUtf8Error { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_filter_error(source: tracing_subscriber::filter::ParseError) -> Error {
    error!("Wrapping error: {}", source);
    Error::TracingFilterError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_subscriber_error(source: tracing::subscriber::SetGlobalDefaultError) -> Error {
    error!("Wrapping error: {}", source);
    Error::TracingSubscriberError { source }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_identifier_error<S>(input: S) -> Error
where
    S: Into<String>,
{
    let input = input.into();
    error!("Invalid Identifier input: {}", input);
    Error::InvalidIdentifierError { input }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_language_tag_error<S>(input: S) -> Error
where
    S: Into<String>,
{
    let input = input.into();
    error!("Invalid LanguageTag input: {}", input);
    Error::InvalidLanguageTagError { input }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_node_kind<S1, S2>(rule: S1, got: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    let rule = rule.into();
    let got = got.into();
    error!("Unexpected node kind; got: {}, in rule: {}", got, rule);
    Error::InvalidNodeKind { rule, got }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn unexpected_node_kind<S1, S2, S3>(rule: S1, expected: S2, got: S3) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    let rule = rule.into();
    let expected = expected.into();
    let got = got.into();
    error!(
        "Invalid node kind; expecting: {}, got: {}, in rule: {}",
        expected, got, rule
    );
    Error::UnexpectedNodeKind {
        rule,
        expected,
        got,
    }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn missing_node_kind<S1, S2>(rule: S1, expected: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    let rule = rule.into();
    let expected = expected.into();
    error!(
        "Missing node kind; expecting: {}, in rule: {}",
        expected, rule
    );
    Error::MissingNodeKind { rule, expected }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_value_for_type<S1, S2>(value: S1, type_name: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    let value = value.into();
    let type_name = type_name.into();
    error!(
        "Invalid value for type; value: {}, type: {}",
        value, type_name
    );
    Error::InvalidValueForType { value, type_name }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn module_file_not_found<S>(name: S) -> Error
where
    S: Into<String>,
{
    let name = name.into();
    error!("Could not resolve module name to a file; name: {}", name);
    Error::ModuleFileNotFound { name }
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn module_parse_error<S1, S2>(
    node_name: S1,
    start: usize,
    end: usize,
    rule: Option<S2>,
) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    let node_name = node_name.into();
    let rule = rule.map(|s| s.into());
    error!(
        "Error reported parsing module; node name: {} range: {}..{}{}",
        node_name,
        start,
        end,
        if let Some(rule) = &rule {
            format!(", in rule: {}", rule)
        } else {
            String::new()
        }
    );
    Error::ModuleParseError {
        rule,
        node_name,
        start,
        end,
    }
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
                Self::TracingFilterError { source } => format!(
                    "A error occurred parsing a tracing filter; source: {}",
                    source
                ),
                Self::TracingSubscriberError { source } => format!(
                    "A error occurred setting the tracing subscriber; source: {}",
                    source
                ),
                Self::InvalidIdentifierError { input } => format!(
                    "Provided input is not a valid identifier; input: {:?}",
                    input
                ),
                Self::InvalidLanguageTagError { input } => format!(
                    "Provided input is not a valid language tag; input: {:?}",
                    input
                ),
                Self::InvalidNodeKind { rule, got } =>
                    format!("Unexpected node kind; got: {}, in rule: {}", got, rule),
                Self::UnexpectedNodeKind {
                    rule,
                    expected,
                    got,
                } => format!(
                    "Invalid node kind; expecting: {}, got: {}, in rule: {}",
                    expected, got, rule
                ),
                Self::InvalidValueForType { value, type_name } => format!(
                    "Invalid value for type; value: {}, type: {}",
                    value, type_name
                ),
                Self::MissingNodeKind { rule, expected } => format!(
                    "Missing node kind; expecting: {}, in rule: {}",
                    expected, rule
                ),
                Self::ModuleFileNotFound { name } =>
                    format!("Could not resolve module name to a file; name: {}", name),
                Self::ModuleParseError {
                    rule,
                    node_name,
                    start,
                    end,
                } => format!(
                    "Error reported parsing module; node name: {} range: {}..{}{}",
                    node_name,
                    start,
                    end,
                    if let Some(rule) = rule {
                        format!(", in rule: {}", rule)
                    } else {
                        String::new()
                    }
                ),
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
