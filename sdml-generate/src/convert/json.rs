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
pub struct Generator {
    format_options: GeneratorOptions,
}

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
    fn with_format_options(mut self, format: GeneratorOptions) -> Self {
        self.format_options = format;
        self
    }

    fn format_options(&self) -> &GeneratorOptions {
        &self.format_options
    }

    fn write<W>(&mut self, module: &Module, _: &ModuleCache, writer: &mut W) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        if self.format_options.pretty_print {
            Ok(serde_json::to_writer_pretty(writer, module).map_err(into_generator_error)?)
        } else {
            Ok(serde_json::to_writer(writer, module).map_err(into_generator_error)?)
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline(always)]
fn into_generator_error(e: serde_json::Error) -> Error {
    crate::errors::into_generator_error("JSON", e)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
