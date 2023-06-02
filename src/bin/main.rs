use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use sdml::error::{tracing_filter_error, tracing_subscriber_error};
use sdml::model::resolve::Resolver;
use sdml::model::Identifier;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

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
    /// Highlight an SDML source file
    Highlight(Highlight),
    /// Extract tags from an SDML Module
    Tags(Tags),
    /// Convert SDML modules into other formats
    Convert(Convert),
    /// Draw diagrams from SDML modules
    Draw(Draw),
}

#[derive(Args, Debug)]
struct FileArgs {
    /// File name to write to, if not provided will write to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// The path to use as the IRI base for modules
    #[arg(short, long)]
    base_path: Option<PathBuf>,

    /// SDML module to convert
    module: Identifier,
}

#[derive(Args, Debug)]
struct Highlight {
    /// Format to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = HighlightFormat::Ansi)]
    output_format: HighlightFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum HighlightFormat {
    /// ANSI escape for console
    Ansi,
    /// HTML pre-formatted element
    Html,
    /// HTML stand-alone document
    HtmlStandalone,
}

#[derive(Args, Debug)]
struct Tags {
    #[command(flatten)]
    files: FileArgs,
}

#[derive(Args, Debug)]
struct Convert {
    /// Format to convert into (org, rdf, sexpr)
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: ConvertFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum ConvertFormat {
    /// Emacs Org Mode Documentation
    Org,
    /// RDF Abstract Model
    Rdf,
    /// S-Expressions
    SExpr,
}

#[derive(Args, Debug)]
struct Draw {
    /// Diagram to draw
    #[arg(short, long)]
    #[arg(value_enum)]
    diagram: DrawDiagram,

    /// Format for diagram result
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: Option<DiagramFormat>,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum DrawDiagram {
    /// Concept Overview
    Concepts,
    /// Entity Relationship
    EntityRelationship,
    /// UML Class
    UmlClass,
}

#[derive(ValueEnum, Clone, Debug)]
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

    let filter = EnvFilter::from_default_env().add_directive(
        format!("{}={}", module_path!(), log_level)
            .parse()
            .map_err(tracing_filter_error)?,
    );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).map_err(tracing_subscriber_error)?;

    info!("Log level set to `LevelFilter::{:?}`", log_level);

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers
// ------------------------------------------------------------------------------------------------

trait Execute {
    fn execute(&self) -> Result<(), MainError>;
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Router
// ------------------------------------------------------------------------------------------------

impl Execute for Commands {
    fn execute(&self) -> Result<(), MainError> {
        match self {
            Commands::Highlight(cmd) => cmd.execute(),
            Commands::Tags(cmd) => cmd.execute(),
            Commands::Convert(cmd) => cmd.execute(),
            Commands::Draw(cmd) => cmd.execute(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ File Args
// ------------------------------------------------------------------------------------------------

impl FileArgs {
    fn resolver(&self) -> Resolver {
        if let Some(base) = &self.base_path {
            Resolver::default().prepend(base.clone())
        } else {
            Resolver::default()
        }
    }

    fn output_writer(&self) -> Result<Box<dyn std::io::Write>, MainError> {
        if let Some(output_file) = &self.output_file {
            Ok(Box::new(std::fs::File::create(output_file)?))
        } else {
            Ok(Box::new(std::io::stdout()))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Highlight
// ------------------------------------------------------------------------------------------------

impl Execute for Highlight {
    fn execute(&self) -> Result<(), MainError> {
        let resolver = self.files.resolver();
        let source = resolver.resolve_module_source(&self.files.module)?;

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            HighlightFormat::Ansi => {
                sdml::actions::highlight::write_highlighted_as_ansi(source, &mut writer)?
            }
            HighlightFormat::Html => {
                sdml::actions::highlight::write_highlighted_as_html(source, &mut writer, false)?
            }
            HighlightFormat::HtmlStandalone => {
                sdml::actions::highlight::write_highlighted_as_html(source, &mut writer, true)?
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Tags
// ------------------------------------------------------------------------------------------------

impl Execute for Tags {
    fn execute(&self) -> Result<(), MainError> {
        let resolver = self.files.resolver();
        let file_name = resolver.resolve_module_path(&self.files.module)?;
        let module = sdml::model::parse::parse_file(&file_name)?;

        info!(
            "loaded module: {}, is_complete: {}",
            module.name(),
            module.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        sdml::actions::tags::write_tags(&module, &mut writer)?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Converter
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), MainError> {
        let resolver = self.files.resolver();
        let file_name = resolver.resolve_module_path(&self.files.module)?;
        let model = sdml::model::parse::parse_file(&file_name)?;

        info!(
            "loaded module: {}, is_complete: {}",
            model.name(),
            model.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            ConvertFormat::Org => {
                sdml::convert::org::write_as_org(&model, &mut writer)?;
            }
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
        let resolver = self.files.resolver();
        let file_name = resolver.resolve_module_path(&self.files.module)?;
        let model = sdml::model::parse::parse_file(&file_name)?;

        info!(
            "loaded module: {}, is_complete: {}",
            model.name(),
            model.is_complete()
        );

        let format = self.output_format.clone().unwrap_or_default().into();

        match self.diagram {
            DrawDiagram::Concepts => {
                if let Some(path) = &self.files.output_file {
                    sdml::draw::concepts::concept_diagram_to_file(&model, path, format)?;
                } else {
                    sdml::draw::concepts::print_concept_diagram(&model, format)?;
                }
            }
            DrawDiagram::EntityRelationship => {
                if let Some(path) = &self.files.output_file {
                    sdml::draw::erd::erd_diagram_to_file(&model, path, format)?;
                } else {
                    sdml::draw::erd::print_erd_diagram(&model, format)?;
                }
            }
            DrawDiagram::UmlClass => {
                if let Some(path) = &self.files.output_file {
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

// TODO: default?

impl Display for ConvertFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Org => "org",
                Self::Rdf => "rdf",
                Self::SExpr => "s-expr",
            }
        )
    }
}

impl FromStr for ConvertFormat {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "org" | "doc" => Ok(Self::Org),
            "rdf" | "ttl" | "turtle" => Ok(Self::Rdf),
            "s-expr" => Ok(Self::SExpr),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Kind
// ------------------------------------------------------------------------------------------------

// TODO: default?

impl Display for DrawDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Concepts => "concepts",
                Self::EntityRelationship => "entity-relationship",
                Self::UmlClass => "uml-class",
            }
        )
    }
}

impl FromStr for DrawDiagram {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "concepts" => Ok(Self::Concepts),
            "entity-relationship" | "erd" => Ok(Self::EntityRelationship),
            "uml-class" | "uml" => Ok(Self::UmlClass),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Format
// ------------------------------------------------------------------------------------------------

impl Default for HighlightFormat {
    fn default() -> Self {
        Self::Ansi
    }
}

impl Display for HighlightFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ansi => "ansi",
                Self::Html => "html",
                Self::HtmlStandalone => "html-standalone",
            }
        )
    }
}

impl FromStr for HighlightFormat {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ansi" => Ok(Self::Ansi),
            "html" => Ok(Self::Html),
            "html-standalone" => Ok(Self::HtmlStandalone),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for DiagramFormat {
    fn default() -> Self {
        Self::Source
    }
}

impl Display for DiagramFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Source => "src",
                Self::Jpeg => "jpg",
                Self::Png => "png",
                Self::Svg => "svg",
            }
        )
    }
}

impl FromStr for DiagramFormat {
    type Err = sdml::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "src" | "source" => Ok(Self::Source),
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
