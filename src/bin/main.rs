use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;
use sdml::error::{tracing_filter_error, tracing_subscriber_error};
use std::path::PathBuf;
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Command-Line Arguments
// ------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert model files into other formats
    Convert(Convert),
    /// Draw diagrams from models
    Draw(Draw),
}

#[derive(Args, Debug)]
struct Convert {
    /// Format to convert into (rdf, sexpr)
    #[arg(short='f', long)]
    output_format: ConvertFormat,

    /// File name to write to, if not provided will write to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// SDML model file to convert from
    input_file: PathBuf,
}

#[derive(Clone, Debug)]
enum ConvertFormat {
    /// RDF
    Rdf,
    /// S-Expressions
    SExpr,
}

#[derive(Args, Debug)]
struct Draw {
    /// Diagram to draw (concepts, entities, uml)
    #[arg(short, long)]
    diagram: DrawDiagram,

    /// Format for diagram result (source, jpeg, png, svg)
    #[arg(short='f', long)]
    output_format: Option<DiagramFormat>,

    /// File name to write to, if not provided will write to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// SDML model file to convert from
    input_file: PathBuf,
}

#[derive(Clone, Debug)]
enum DrawDiagram {
    /// Concept Overview
    Concepts,
    /// Entity Relationship
    EntityRelationship,
    /// UML Class
    UmlClass,
}

#[derive(Clone, Debug)]
enum DiagramFormat {
    Source,
    Jpeg,
    Png,
    Svg,
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

type MainError = Box<dyn std::error::Error>;

fn main() -> Result<(), MainError> {
    let cli = Cli::parse();

    init_logging(cli.verbose)?;

    cli.command.execute()
}

// ------------------------------------------------------------------------------------------------
// Main ❱ Logging
// ------------------------------------------------------------------------------------------------

fn init_logging(verbosity: Verbosity) -> Result<(), MainError> {
    let log_level = verbosity.log_level_filter();

    let filter = EnvFilter::from_default_env()
        .add_directive(
            format!("{}={}", module_path!(), log_level)
                .parse()
                .map_err(|e| tracing_filter_error(e))?);
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| tracing_subscriber_error(e))?;

    info!("Log level set to `LevelFilter::{:?}`", log_level);

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers
// ------------------------------------------------------------------------------------------------

trait Execute {
    fn execute(&self) -> Result<(), MainError>;
}

fn output_writer(file: &Option<PathBuf>) -> Result<Box<dyn std::io::Write>, MainError> {
    if let Some(file) = file {
        Ok(Box::new(std::fs::File::create(file)?))
    } else {
        Ok(Box::new(std::io::stdout()))
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Router
// ------------------------------------------------------------------------------------------------

impl Execute for Commands {
    fn execute(&self) -> Result<(), MainError> {
        match self {
            Commands::Convert(cmd) => cmd.execute(),
            Commands::Draw(cmd) => cmd.execute(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Converter
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), MainError> {
        let model = sdml::parse_file(&self.input_file)?;
        let mut writer = output_writer(&self.output_file)?;

        match self.output_format {
            ConvertFormat::Rdf => {
                sdml::convert::rdf::write_as_rdf(&model, &mut writer)?;
            }
            ConvertFormat::SExpr => {
                sdml::convert::sexpr::write_as_sexpr(&model, &mut writer)?;
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Draw
// ------------------------------------------------------------------------------------------------

impl Execute for Draw {
    fn execute(&self) -> Result<(), MainError> {
        let model = sdml::parse_file(&self.input_file)?;
        let format = self.output_format.clone().unwrap_or_default().into();

        match self.diagram {
            DrawDiagram::Concepts => {
                if let Some(path) = &self.output_file {
                    sdml::draw::concepts::concept_diagram_to_file(&model, path, format)?;
                } else {
                    sdml::draw::concepts::print_concept_diagram(&model, format)?;
                }
            }
            DrawDiagram::EntityRelationship => {
                if let Some(path) = &self.output_file {
                    sdml::draw::erd::erd_diagram_to_file(&model, path, format)?;
                } else {
                    sdml::draw::erd::print_erd_diagram(&model, format)?;
                }
            }
            DrawDiagram::UmlClass => {
                if let Some(path) = &self.output_file {
                    sdml::draw::uml::uml_diagram_to_file(&model, path, format)?;
                } else {
                    sdml::draw::uml::print_uml_diagram(&model, format)?;
                }
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Conversion
// ------------------------------------------------------------------------------------------------

impl Display for ConvertFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Rdf => "rdf",
            Self::SExpr => "sexpr",
        })
    }
}

impl FromStr for ConvertFormat {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rdf" | "ttl" | "turtle" => Ok(Self::Rdf),
            "sexpr" | "s-expr" => Ok(Self::SExpr),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Kind
// ------------------------------------------------------------------------------------------------

impl Display for DrawDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Concepts => "concepts",
            Self::EntityRelationship => "entities",
            Self::UmlClass => "uml",
        })
    }
}

impl FromStr for DrawDiagram {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "concepts" => Ok(Self::Concepts),
            "entities" | "entity-relations" | "erd" => Ok(Self::EntityRelationship),
            "uml" | "uml-class" => Ok(Self::UmlClass),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Format
// ------------------------------------------------------------------------------------------------

impl Default for DiagramFormat {
    fn default() -> Self {
        Self::Source
    }
}

impl Display for DiagramFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Source => "src",
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Svg => "svg",
        })
    }
}

impl FromStr for DiagramFormat {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "src" => Ok(Self::Source),
            "jpg" | "jpeg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "svg" => Ok(Self::Svg),
            _ => panic!(),
        }
    }
}

impl From<DiagramFormat> for sdml::draw::OutputFormat {
    fn from(v: DiagramFormat) -> Self {
        match v {
            DiagramFormat::Source => sdml::draw::OutputFormat::Source,
            DiagramFormat::Jpeg => sdml::draw::OutputFormat::ImageJpeg,
            DiagramFormat::Png => sdml::draw::OutputFormat::ImagePng,
            DiagramFormat::Svg => sdml::draw::OutputFormat::ImageSvg,
        }
    }
}
