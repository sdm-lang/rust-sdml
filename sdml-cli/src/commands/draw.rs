use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
};
use sdml_error::Error;
use sdml_generate::{GenerateToFile, GenerateToWriter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Draw diagrams from a module
#[derive(Args, Debug)]
pub(crate) struct Command {
    /// Diagram kind to draw
    #[arg(short, long)]
    #[arg(value_enum)]
    diagram: DrawDiagram,

    /// Format for diagram result
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: Option<OutputFormat>,

    #[command(flatten)]
    files: super::FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DrawDiagram {
    /// Concept Overview
    Concepts,
    /// Entity Relationship
    EntityRelationship,
    /// UML Class
    UmlClass,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OutputFormat {
    Source,
    Jpeg,
    Png,
    Svg,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module: &Module, cache: &ModuleCache, _| {
            let format = self.output_format.unwrap_or_default().into();

            match self.diagram {
                DrawDiagram::Concepts => {
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default();
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(module, cache, path, format)?;
                    } else {
                        generator.write_in_format(module, cache, &mut std::io::stdout(), format)?;
                    }
                }
                DrawDiagram::EntityRelationship => {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(module, cache, path, format)?;
                    } else {
                        generator.write_in_format(module, cache, &mut std::io::stdout(), format)?;
                    }
                }
                DrawDiagram::UmlClass => {
                    let mut generator = sdml_generate::draw::uml::UmlDiagramGenerator::default();
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(module, cache, path, format)?;
                    } else {
                        println!("Sorry, writing UML diagrams requires an explicit output file");
                    }
                }
            }

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Source
    }
}

impl From<OutputFormat> for sdml_generate::draw::OutputFormat {
    fn from(v: OutputFormat) -> Self {
        match v {
            OutputFormat::Source => Self::Source,
            OutputFormat::Jpeg => Self::ImageJpeg,
            OutputFormat::Png => Self::ImagePng,
            OutputFormat::Svg => Self::ImageSvg,
        }
    }
}
