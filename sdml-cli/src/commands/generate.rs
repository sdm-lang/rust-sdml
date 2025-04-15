use clap::Args;
use sdml_core::{load::ModuleLoader, store::ModuleStore};
use sdml_errors::Error;
use sdml_tera::{make_engine_from, render_module_to};
use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Generate content using Tera templates
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[command(flatten)]
    files: super::FileArgs,

    /// A file glob for loading templates
    #[arg(short = 'g', long)]
    #[clap(value_parser, default_value = "templates/**/*.md")]
    template_glob: String,

    /// The file name (not path) to the initial template
    #[arg(short = 'n', long)]
    #[clap(value_parser)]
    template_name: String,
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

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module, _, _| {
            let engine = make_engine_from(&self.template_glob)?;

            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            render_module_to(&engine, module, None, &self.template_name, &mut writer)?;

            Ok(ExitCode::SUCCESS)
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
