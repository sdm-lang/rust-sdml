use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{load::ModuleLoader, store::ModuleStore};
use sdml_errors::Error;
use sdml_generate::actions::tags::write_ctags;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Extract tags from a module.
///
/// This
///
/// - CTags ::
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputFormat::CTags)]
    output_format: OutputFormat,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Format to convert into
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OutputFormat {
    /// ctags format
    CTags,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module: &Module, _, _| {
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            match self.output_format {
                OutputFormat::CTags => write_ctags(module, module.source_file(), &mut writer)?,
            }

            Ok(ExitCode::SUCCESS)
        });
    }
}
