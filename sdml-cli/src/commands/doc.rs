use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
    model::modules::Module,
    model::HasName,
};
use sdml_errors::Error;
use sdml_generate::Generator;
use sdml_parse::load::FsModuleLoader;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Produce structured documentation for a module.
///
/// This command creates structured documentation for a module, and includes annotations,
/// constraints and all definition types. The generated documentation also include diagrams and
/// dependency graphs.
///
/// - Org-Mode (default) :: Create an Emacs org-mode formatted file. This format allows all
///   content to be written into a single file with export options to HTML, LaTeX, Word, PDF and
///   more.
///
/// - Markdown :: Create a markdown formatted file, this file uses GitHub-flavored markdown to
///   allow for some better content formatting than CommonMark.
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputFormat::OrgMode)]
    output_format: OutputFormat,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Markup format to generate
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OutputFormat {
    /// Markdown
    Markdown,
    /// Emacs org-mode
    OrgMode,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(
            self,
            |module: &Module, cache: &ModuleCache, loader: &FsModuleLoader| {
                match self.output_format {
                    OutputFormat::OrgMode => {
                        let source = loader.get_source_by_name(module.name());
                        let mut generator =
                            sdml_generate::convert::doc::org_mode::DocumentationGenerator::new(
                                source,
                                Default::default(),
                            );
                        self.write_org(module, cache, &mut generator)?;
                    }
                    OutputFormat::Markdown => {}
                }

                Ok(ExitCode::SUCCESS)
            }
        );
    }
}

impl Command {
    fn write_org(
        &self,
        model: &Module,
        cache: &ModuleCache,
        generator: &mut sdml_generate::convert::doc::org_mode::DocumentationGenerator,
    ) -> Result<(), Error> {
        let mut output = self.files.output.clone();
        let mut writer = output.lock();

        generator.generate(model, cache, None, &mut writer)
    }
}
