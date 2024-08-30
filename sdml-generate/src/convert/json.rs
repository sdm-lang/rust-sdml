/*!
This module provides a generator that creates the JSON representation of a module given its
in-memory representation.

 */

use crate::Generator;
use sdml_core::error::Error;
use sdml_core::model::modules::Module;
use sdml_core::store::ModuleStore;
use std::io::Write;
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Generator for the JSON representation of a module's in-memory model.
///
#[derive(Debug, Default)]
pub struct JsonGenerator {}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct JsonGeneratorOptions {
    pretty_print: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl JsonGeneratorOptions {
    pub fn pretty_print(self, pretty_print: bool) -> Self {
        Self { pretty_print }
    }
}

// ------------------------------------------------------------------------------------------------

impl Generator for JsonGenerator {
    type Options = JsonGeneratorOptions;

    fn generate_with_options<W>(
        &mut self,
        module: &Module,
        _: &impl ModuleStore,
        options: Self::Options,
        _: Option<PathBuf>,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        if options.pretty_print {
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
