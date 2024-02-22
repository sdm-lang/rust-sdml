use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{cache::ModuleStore, load::ModuleLoader};
use sdml_error::Error;
use sdml_generate::GenerateToWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// View formatted module source code
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = SourceGenerationLevel::Full)]
    level: SourceGenerationLevel,

    /// Set the number of spaces for indentation
    #[arg(short = 's', long)]
    #[arg(default_value = "2")]
    indent_spaces: usize,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Determine the amount of source to generate
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum SourceGenerationLevel {
    /// Top-level definitions, incomplete
    Definitions,
    /// Top-level definitions and members, incomplete
    Members,
    /// Full source
    Full,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module: &Module, cache, _| {
            let mut writer = self.files.output_writer()?;

            let mut generator = sdml_generate::convert::source::SourceGenerator::default();
            generator.write_in_format(module, cache, &mut writer, self.level.into())?;

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------

impl From<SourceGenerationLevel> for sdml_generate::convert::source::SourceGenerationLevel {
    fn from(v: SourceGenerationLevel) -> Self {
        match v {
            SourceGenerationLevel::Full => Self::Full,
            SourceGenerationLevel::Definitions => Self::Definitions,
            SourceGenerationLevel::Members => Self::Members,
        }
    }
}
