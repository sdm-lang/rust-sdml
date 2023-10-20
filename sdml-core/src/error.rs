/*!
Provides the crate's Error and Result types as well as helper functions.
 */

use std::fmt::{Debug, Display};
use tracing::error;

use crate::model::Span;

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
    CodespanReportingError {
        source: codespan_reporting::files::Error,
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
        span: Span,
    },
    MissingNodeKind {
        rule: String,
        expected: String,
    },
    MissingNodeVariable {
        rule: String,
        expected_name: String,
        expected_kind: String,
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
        span: Span,
    },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

macro_rules! report_and_return {
    ($err: expr) => {
        let err = $err;
        error!("{}", err);
        return err;
    };
}

/// Construct an Error from the provided source.
#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    report_and_return!(Error::IoError { source });
}

/// Construct an Error from the provided source.
#[inline]
pub fn utf8_error(source: core::str::Utf8Error) -> Error {
    report_and_return!(Error::Utf8Error { source });
}

/// Construct an Error from the provided source.
#[inline]
pub fn from_utf8_error(source: std::string::FromUtf8Error) -> Error {
    report_and_return!(Error::FromUtf8Error { source });
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_filter_error(source: tracing_subscriber::filter::ParseError) -> Error {
    report_and_return!(Error::TracingFilterError { source });
}

/// Construct an Error from the provided source.
#[inline]
pub fn tracing_subscriber_error(source: tracing::subscriber::SetGlobalDefaultError) -> Error {
    report_and_return!(Error::TracingSubscriberError { source });
}

/// Construct an Error from the provided source.
#[inline]
pub fn codespan_reporting_error(source: codespan_reporting::files::Error) -> Error {
    report_and_return!(Error::CodespanReportingError { source });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_identifier_error<S>(input: S) -> Error
where
    S: Into<String>,
{
    report_and_return!(Error::InvalidIdentifierError {
        input: input.into()
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_language_tag_error<S>(input: S) -> Error
where
    S: Into<String>,
{
    report_and_return!(Error::InvalidLanguageTagError {
        input: input.into()
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_node_kind<S1, S2>(rule: S1, got: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    report_and_return!(Error::InvalidNodeKind {
        rule: rule.into(),
        got: got.into()
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn unexpected_node_kind<S1, S2, S3>(rule: S1, expected: S2, got: S3, span: Span) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    report_and_return!(Error::UnexpectedNodeKind {
        rule: rule.into(),
        expected: expected.into(),
        got: got.into(),
        span,
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn missing_node_kind<S1, S2>(rule: S1, expected: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    report_and_return!(Error::MissingNodeKind {
        rule: rule.into(),
        expected: expected.into()
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn missing_node_variable<S1, S2, S3>(rule: S1, expected_name: S2, expected_kind: S3) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    report_and_return!(Error::MissingNodeVariable {
        rule: rule.into(),
        expected_name: expected_name.into(),
        expected_kind: expected_kind.into(),
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn invalid_value_for_type<S1, S2>(value: S1, type_name: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    report_and_return!(Error::InvalidValueForType {
        value: value.into(),
        type_name: type_name.into()
    });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn module_file_not_found<S>(name: S) -> Error
where
    S: Into<String>,
{
    report_and_return!(Error::ModuleFileNotFound { name: name.into() });
}

/// Construct an invalid value Error from the provided input.
#[inline]
pub fn module_parse_error<S1, S2>(node_name: S1, span: Span, rule: Option<S2>) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    report_and_return!(Error::ModuleParseError {
        rule: rule.map(|s| s.into()),
        node_name: node_name.into(),
        span,
    });
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------
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
                Self::TracingFilterError { source } => format!(
                    "A error occurred parsing a tracing filter; source: {source}"),
                Self::TracingSubscriberError { source } => format!(
                    "A error occurred setting the tracing subscriber; source: {source}"),
                Self::CodespanReportingError { source } => format!(
                    "An error occurred formatting codespan reports; source: {source}"),
                Self::InvalidIdentifierError { input } => format!(
                    "Provided input is not a valid identifier; input: {input:?}"),
                Self::InvalidLanguageTagError { input } => format!(
                    "Provided input is not a valid language tag; input: {input:?}"),
                Self::InvalidNodeKind { rule, got } =>
                    format!("Unexpected node kind; got: {got}, in rule: {rule}"),
                Self::UnexpectedNodeKind {
                    rule,
                    expected,
                    got,
                    span,
                } => format!(
                    "Invalid node kind; expecting: {expected}, got: {got}, in rule: {rule}, span: {span}"),
                Self::InvalidValueForType { value, type_name } => format!(
                    "Invalid value for type; value: {value}, type: {type_name}"),
                Self::MissingNodeKind { rule, expected } => format!(
                    "Missing node kind; expecting: {expected}, in rule: {rule}"),
                Self::MissingNodeVariable { rule, expected_name, expected_kind } => format!(
                    "Missing node variable; expecting variable: {expected_name}, kind {expected_kind}, in rule: {rule}"),
                Self::ModuleFileNotFound { name } =>
                    format!("Could not resolve module name to a file; name: {name}"),
                Self::ModuleParseError {
                    rule,
                    node_name,
                    span,
                } => format!(
                    "Error reported parsing module; node name: {node_name} span: {span}{}",
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

impl<T> From<Error> for Result<T> {
    fn from(value: Error) -> Self {
        Err(value)
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

impl From<codespan_reporting::files::Error> for Error {
    fn from(source: codespan_reporting::files::Error) -> Self {
        codespan_reporting_error(source)
    }
}
