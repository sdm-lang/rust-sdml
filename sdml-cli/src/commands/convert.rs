use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    load::ModuleLoader,
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::Error;

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

use objio::{HasOptions, ObjectWriter};
use sexpr_out::writer::LanguageStyle;
use sexpr_out::{Options as SExprOptions, Value as SExprValue, Writer as SExprWriter};

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module: &Module, cache: &InMemoryModuleCache, _| {
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            match self.output_format {
                ConvertFormat::Rdf => {
                    sdml_rdf::write::write_module_with_options(
                        &mut writer,
                        module,
                        cache,
                        Default::default(),
                        Default::default(),
                    )?;
                }
                ConvertFormat::Json | ConvertFormat::JsonPretty => {
                    sdml_json::write::write_module_with_options(
                        &mut writer,
                        module,
                        cache,
                        sdml_json::WriteOptions::default()
                            .with_pretty_printing(self.output_format == ConvertFormat::JsonPretty),
                    )?;
                }
                ConvertFormat::SExpr => {
                    let sexpr_writer = SExprWriter::default().pretty_printed(true).with_options(
                        SExprOptions::default()
                            .with_line_width(80)
                            .with_style(LanguageStyle::Racket),
                    );
                    sexpr_writer
                        .write(&mut writer, &SExprValue::Bool(true))
                        .map_err(|e| Error::GeneratorError {
                            name: "rdf".to_string(),
                            message: e.to_string(),
                        })?;
                }
            }

            Ok(ExitCode::SUCCESS)
        });
    }
}
