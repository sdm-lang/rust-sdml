use clap::{Args, ValueEnum};
use sdml_core::load::{ModuleLoader, ModuleResolver};
use sdml_core::stdlib;
use sdml_error::Error;
use sdml_parse::load::FsModuleLoader;
use std::io::Read;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Syntax highlight a module source
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputFormat::Ansi)]
    output_format: OutputFormat,

    #[command(flatten)]
    files: super::FileArgs,
}

/// Format to convert into
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OutputFormat {
    /// ANSI escape for console
    Ansi,
    /// HTML pre-formatted element
    Html,
    /// HTML stand-alone document
    HtmlStandalone,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<(), Error> {
        let loader = FsModuleLoader::default();
        let resolver = loader.resolver();

        let source = if let Some(module_name) = &self.files.module {
            if stdlib::is_library_module(module_name) {
                println!("Sorry, can't currently highlight stdlib modules");
                return Ok(());
            } else {
                let resource =
                    resolver.name_to_resource(module_name, loader.get_file_id(module_name))?;
                let file_path = resource.to_file_path().unwrap();
                std::fs::read_to_string(file_path)?
            }
        } else {
            let mut input = self.files.input.clone();
            if input.path().is_file() {
                let mut reader = input.lock();
                let mut source = String::new();
                reader.read_to_string(&mut source)?;
                source
            } else {
                println!("Error: the input file `{}` does not exist.", input.path());
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
        };

        let mut output = self.files.output.clone();
        let mut writer = output.lock();

        match self.output_format {
            OutputFormat::Ansi => {
                sdml_generate::actions::highlight::write_highlighted_as_ansi(source, &mut writer)?
            }
            OutputFormat::Html => sdml_generate::actions::highlight::write_highlighted_as_html(
                source,
                &mut writer,
                false,
            )?,
            OutputFormat::HtmlStandalone => {
                sdml_generate::actions::highlight::write_highlighted_as_html(
                    source,
                    &mut writer,
                    true,
                )?
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Ansi
    }
}
