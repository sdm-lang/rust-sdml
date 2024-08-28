/*!
One-line description.

More detailed description, with

# Example

End of file during parsingSymbol’s value as variable is void: rustEnd of file during parsing

 */

use sdml_errors::Error;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn into_generator_error<S, E>(name: S, error: E) -> Error
where
    S: Into<String>,
    E: std::error::Error,
{
    generator_error(name, error.to_string())
}

pub(crate) fn generator_error<S1, S2>(name: S1, message: S2) -> Error
where
    S1: Into<String>,
    S2: Into<String>,
{
    Error::GeneratorError {
        name: name.into(),
        message: message.into(),
    }
}
