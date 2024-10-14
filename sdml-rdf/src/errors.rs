/*!
Provides error handling helpers.

 */

use sdml_core::model::identifiers::Identifier;
use sdml_errors::Error;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an Error from the provided source.
#[inline]
pub(crate) fn missing_base_uri_error(module: &Identifier) -> Error {
    Error::GeneratorError {
        name: "RDF".into(),
        message: format!("Module `{module}` has no base URI"),
    }
}

/// Construct an Error from the provided source.
#[inline]
pub(crate) fn module_not_loaded_error(module: &Identifier) -> Error {
    Error::GeneratorError {
        name: "RDF".into(),
        message: format!("Module `{module}` not found in provided cache"),
    }
}
