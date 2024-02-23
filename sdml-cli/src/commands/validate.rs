use clap::{Args, ValueEnum};
use sdml_core::model::check::terms::{default_term_set, validate_module_terms};
use sdml_core::model::{modules::Module, HasName};
use sdml_core::{cache::ModuleStore, load::ModuleLoader};
use sdml_error::diagnostics::{
    reporter::{CompactStreamReporter, Reporter, StandardStreamReporter},
    SeverityFilter,
};
use sdml_error::Error;
use sdml_parse::load::FsModuleLoader;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Validate that a module is correct.
///
/// This command provides deep validation of a module's content, including errors, warnings, and
/// linter-like advice. Checks are run not only on the initial module, but it's transitively
/// loaded dependencies.
///
/// By default the command only shows diagnostics with severity `bug` and `error`, but `warning`,
/// `notes`, and `help` can be output with the `--level` argument. This argument also takes the
/// values `none` and `all`.
///
/// ```text
/// ❯ sdml validate --level all -i examples/errors/i0506.sdm
/// note[I0506]: identifier not using preferred casing
///   ┌─ examples/errors/i0506.sdm:1:8
///   │
/// 1 │ module Example <https://example.com/api> is
///   │        ^^^^^^^ this identifier
///   │
///   = expected snake case (snake_case)
///   = help: for more details, see <https://sdml.io/errors/#I0506>
///
/// note[I0506]: identifier not using preferred casing
///   ┌─ examples/errors/i0506.sdm:3:13
///   │
/// 3 │   structure access_record is
///   │             ^^^^^^^^^^^^^ this identifier
///   │
///   = expected upper camel case (UpperCamelCase)
///   = help: for more details, see <https://sdml.io/errors/#I0506>
/// ```
///
/// The `check-constraints` option turns on (it's default is off) the checking of constraints
/// for correctness.
///
/// Additionally, a `short-form` option will generate diagnostics using a CSV format that is
/// easier for tools to parse. The fields in this format are: severity, file name, start line,
/// start column, end line, end column, error code, and message.
///
/// ```text
/// ❯ sdml validate --level all --short-form -i examples/errors/i0506.sdm
/// note,examples/errors/i0506.sdm,1,8,1,15,I0506,identifier not using preferred casing
/// note,examples/errors/i0506.sdm,3,13,3,26,I0506,identifier not using preferred casing
/// ```
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DiagnosticLevel::Warnings)]
    level: DiagnosticLevel,

    /// Enable the checking of constraints in the model
    #[arg(short = 'c', long, default_value = "false")]
    check_constraints: bool,

    /// Enable the short form (CSV) output
    #[arg(short = 's', long, default_value = "false")]
    short_form: bool,

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
        let reporter: Box<dyn Reporter> = if self.short_form {
            Box::<CompactStreamReporter>::default()
        } else {
            Box::<StandardStreamReporter>::default()
        };
        call_with_module!(
            self,
            reporter,
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
