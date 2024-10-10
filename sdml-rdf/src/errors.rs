/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use std::fmt::{Debug, Display};

use sdml_core::model::identifiers::Identifier;

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
    MissingBaseUri {
        module: Identifier,
    },
    ModuleNotLoaded {
        module: Identifier,
    },
}
impl Error {
    pub(crate) fn missing_base_uri_error(name: &str) {
        todo!()
    }
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
pub fn missing_base_uri_error(module: &Identifier) -> Error {
    Error::MissingBaseUri {
        module: module.clone(),
    }
}

/// Construct an Error from the provided source.
#[inline]
pub fn module_not_loaded_error(module: &Identifier) -> Error {
    Error::ModuleNotLoaded {
        module: module.clone(),
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
                Self::IoError { source } => format!("An I/O error occurred; source: {}.", source),
                Self::MissingBaseUri { module } =>
                    format!("The module '{module}' does not include a base URI value."),
                Self::ModuleNotLoaded { module } =>
                    format!("The module '{module}' is not loaded int the provided store."),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        #[allow(unreachable_patterns)]
        match self {
            Self::IoError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        io_error(source)
    }
}
