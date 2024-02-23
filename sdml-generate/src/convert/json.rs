/*!
This module provides a generator that creates the JSON representation of a module given its
in-memory representation.

 */

use crate::GenerateToWriter;
use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::model::modules::Module;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generator for the JSON representation of a module's in-memory model.
///
#[derive(Debug, Default)]
pub struct Generator {}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GeneratorOptions {
    pretty_print: bool,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GeneratorOptions {
    pub fn pretty_printer() -> Self {
        Self { pretty_print: true }
    }

    pub fn is_pretty_printer(&self) -> bool {
        self.pretty_print
    }
}

// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<GeneratorOptions> for Generator {
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        _: &ModuleCache,
        writer: &mut W,
        options: GeneratorOptions,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        if options.pretty_print {
            Ok(serde_json::to_writer_pretty(writer, module).map_err(to_generator_error)?)
        } else {
            Ok(serde_json::to_writer(writer, module).map_err(to_generator_error)?)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn to_generator_error(e: serde_json::Error) -> Error {
    Error::GeneratorError {
        message: e.to_string(),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
