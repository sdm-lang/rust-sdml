use clap::{Args, ValueEnum};
use sdml_core::model::check::terms::{default_term_set, validate_module_terms};
use sdml_core::model::{modules::Module, HasName};
use sdml_core::{cache::ModuleStore, load::ModuleLoader};
use sdml_error::diagnostics::SeverityFilter;
use sdml_error::Error;
use sdml_parse::load::FsModuleLoader;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Validate a module
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DiagnosticLevel::Warnings)]
    level: DiagnosticLevel,

    /// Enable the checking of constraints in the model
    #[arg(short = 'c', long, default_value = "false")]
    check_constraints: bool,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Set the level of diagnostic messages to report
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DiagnosticLevel {
    /// Turn off reporting
    None,
    /// Implementation bugs
    Bugs,
    /// Module errors
    Errors,
    /// Module warnings
    Warnings,
    /// Module informational notes
    Notes,
    /// Style and other help
    Help,
    /// Turn it all on
    All,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(
            self,
            |module: &Module, cache, loader: &mut FsModuleLoader| {
                loader.set_severity_filter(self.level.into());
                module.validate(cache, loader, self.check_constraints);

                let term_set = default_term_set()?;
                validate_module_terms(module, &term_set, loader);

                loader.reporter_done(Some(module.name().to_string()))?;

                Ok(())
            }
        );
    }
}

impl From<DiagnosticLevel> for SeverityFilter {
    fn from(value: DiagnosticLevel) -> Self {
        match value {
            DiagnosticLevel::None => SeverityFilter::None,
            DiagnosticLevel::Bugs => SeverityFilter::Bug,
            DiagnosticLevel::Errors => SeverityFilter::Error,
            DiagnosticLevel::Warnings => SeverityFilter::Warning,
            DiagnosticLevel::Notes => SeverityFilter::Note,
            DiagnosticLevel::Help => SeverityFilter::Help,
            DiagnosticLevel::All => SeverityFilter::Help,
        }
    }
}
