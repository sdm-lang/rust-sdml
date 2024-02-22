use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
};
use sdml_error::Error;
use sdml_generate::convert::{json, rdf, sexpr};
use sdml_generate::GenerateToWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Convert module into alternate representations
#[derive(Args, Debug)]
pub(crate) struct Command {
    /// Module representation to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: ConvertFormat,

    #[command(flatten)]
    files: super::FileArgs,
}

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
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module: &Module, cache: &ModuleCache, _| {
            let mut writer = self.files.output_writer()?;

            match self.output_format {
                ConvertFormat::Rdf => {
                    let mut generator = rdf::RdfModelGenerator::default();
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(
                            module,
                            cache,
                            path,
                            Default::default(),
                        )?;
                    } else {
                        generator.write_in_format(
                            module,
                            cache,
                            &mut std::io::stdout(),
                            Default::default(),
                        )?;
                    }
                }
                ConvertFormat::Json | ConvertFormat::JsonPretty => {
                    let mut generator = json::Generator::default();
                    let options = if self.output_format == ConvertFormat::JsonPretty {
                        json::GeneratorOptions::pretty_printer()
                    } else {
                        json::GeneratorOptions::default()
                    };
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(module, cache, path, options)?;
                    } else {
                        generator.write_in_format(
                            module,
                            cache,
                            &mut std::io::stdout(),
                            options,
                        )?;
                    }
                }
                ConvertFormat::SExpr => {
                    sexpr::write_as_sexpr(module, &mut writer)?;
                }
            }

            Ok(())
        });
    }
}
