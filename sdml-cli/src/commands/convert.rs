use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    load::ModuleLoader,
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::Error;
use sdml_generate::convert::{json, rdf, sexpr};
use sdml_generate::Generator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Convert a module into an alternate representation.
///
/// This command allows the conversion of a module from the SDML surface syntax into one of a
/// number of alternate representations.
///
/// - RDF :: This uses the surface to RDF mapping defined in the SDML Language Reference. The
///   mapping is normative and stable.
///
/// - JSON :: This is a direct representation of the in-memory model in the Rust package
///   `sdml_core` in JSON. This mapping is non-normative and may change according to any model
///   structure change.
///
/// - S-Expression :: This is a debugging representation, and supported as the underlying
///   tree-sitter library uses s-expressions as a parse-tree visualization.
///
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: ConvertFormat,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Module representation to convert into
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ConvertFormat {
    /// JSON
    Json,
    /// Pretty-printed JSON
    JsonPretty,
    /// RDF Abstract Model
    Rdf,
    /// S-Expressions
    SExpr,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module: &Module, cache: &InMemoryModuleCache, _| {
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            match self.output_format {
                ConvertFormat::Rdf => {
                    let mut generator = rdf::RdfModelGenerator::default();
                    generator.generate(module, cache, None, &mut writer)?;
                }
                ConvertFormat::Json | ConvertFormat::JsonPretty => {
                    let options = json::JsonGeneratorOptions::default()
                        .pretty_print(self.output_format == ConvertFormat::JsonPretty);
                    let mut generator = json::JsonGenerator::default();
                    generator.generate_with_options(module, cache, options, None, &mut writer)?;
                }
                ConvertFormat::SExpr => {
                    let options = sexpr::SExpressionOptions::default();
                    let mut generator = sexpr::SExpressionGenerator::default();
                    generator.generate_with_options(module, cache, options, None, &mut writer)?;
                }
            }

            Ok(ExitCode::SUCCESS)
        });
    }
}
