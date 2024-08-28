use clap::{Args, ValueEnum};
use sdml_core::model::modules::Module;
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
};
use sdml_errors::Error;
use sdml_generate::draw::concepts::ConceptDiagramOptions;
use sdml_generate::draw::erd::ErdDiagramOptions;
use sdml_generate::draw::filter::DiagramContentFilter;
use sdml_generate::draw::uml::UmlDiagramOptions;
use sdml_generate::Generator;
use std::path::PathBuf;
use std::process::ExitCode;

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

    /// File name for a content-filter specification (JSON)
    #[arg(short = 'c', long)]
    content_filter: Option<PathBuf>,

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
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module: &Module, cache: &ModuleCache, _| {
            let format = self.output_format.unwrap_or_default();
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            let content_filter = if let Some(content_filter_file) = &self.content_filter {
                if content_filter_file.exists() {
                    DiagramContentFilter::read_from_file(content_filter_file)?
                } else {
                    eprintln!("Filter file does not exist, path: {content_filter_file:?}");
                    return Ok(ExitCode::FAILURE);
                }
            } else {
                DiagramContentFilter::default()
            };

            match self.diagram {
                DrawDiagram::Concepts => {
                    let options = ConceptDiagramOptions::default()
                        .with_content_filter(content_filter)
                        .with_output_format(format.into());
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default();
                    generator.generate_with_options(module, cache, options, None, &mut writer)?;
                }
                DrawDiagram::EntityRelationship => {
                    let options = ErdDiagramOptions::default()
                        .with_content_filter(content_filter)
                        .with_output_format(format.into());
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    generator.generate_with_options(module, cache, options, None, &mut writer)?;
                }
                DrawDiagram::UmlClass => {
                    let options = UmlDiagramOptions::default()
                        .with_content_filter(content_filter)
                        .with_output_format(format.into());
                    let mut generator = sdml_generate::draw::uml::UmlDiagramGenerator::default();
                    if self.files.output.is_local() {
                        let path = self.files.output.path().to_path_buf();
                        generator.generate_to_file(module, cache, options, &path)?;
                    } else {
                        println!("Sorry, writing UML diagrams requires an explicit output file");
                    }
                }
            }

            Ok(ExitCode::SUCCESS)
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
