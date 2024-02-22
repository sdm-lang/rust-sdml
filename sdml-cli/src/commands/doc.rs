use clap::{Args, ValueEnum};
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
    model::modules::Module,
    model::HasName,
};
use sdml_error::Error;
use sdml_generate::GenerateToWriter;
use sdml_parse::load::FsModuleLoader;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Produce structured doocument a module
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
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(
            self,
            |module: &Module, cache: &ModuleCache, loader: &FsModuleLoader| {
                match self.output_format {
                    OutputFormat::OrgMode => {
                        let source = loader.get_source_by_name(module.name());
                        if let Some(source) = source {
                            let mut generator =
                            sdml_generate::convert::doc::org_mode::DocumentationGenerator::with_source(
                                source.as_ref(),
                            );
                            self.write_org(module, cache, &mut generator)?;
                        } else {
                            let mut generator =
                            sdml_generate::convert::doc::org_mode::DocumentationGenerator::default();
                            self.write_org(module, cache, &mut generator)?;
                        };
                    }
                    OutputFormat::Markdown => {}
                }

                Ok(())
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

        generator.write_in_format(model, cache, &mut writer, Default::default())
    }
}
