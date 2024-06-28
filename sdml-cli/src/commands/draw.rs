use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
};
use sdml_errors::Error;
use sdml_generate::{GenerateToFile, GenerateToWriter};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Draw diagrams from a module.
///
/// This command generates diagrams of a module with different perspectives.
///
/// - Concepts ::
///
/// - Entity-Relationship ::
///
/// - UML Class ::
///
/// The diagrams can be generated in jpeg, png, or svg image formats, or in their source form
/// for further processing or inclusion in other documents.
///
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
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            match self.diagram {
                DrawDiagram::Concepts => {
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default()
                            .with_format_options(format);
                    generator.write(module, cache, &mut writer)?;
                }
                DrawDiagram::EntityRelationship => {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default()
                        .with_format_options(format);
                    generator.write(module, cache, &mut writer)?;
                }
                DrawDiagram::UmlClass => {
                    let mut generator = sdml_generate::draw::uml::UmlDiagramGenerator::default()
                        .with_format_options(format);
                    if self.files.output.is_local() {
                        generator.write_to_file(module, cache, self.files.output.path())?;
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
