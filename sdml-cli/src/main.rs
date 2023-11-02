use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use sdml_core::error::{tracing_filter_error, tracing_subscriber_error};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::HasName;
use sdml_generate::convert::org::OrgFileGenerator;
use sdml_generate::{GenerateToFile, GenerateToWriter};
use sdml_parse::load::{ModuleLoader, ModuleResolver};
use sdml_parse::{ModuleLoader as LoaderTrait, ModuleResolver as ResolverTrait};
use std::fmt::Display;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

// ------------------------------------------------------------------------------------------------
// Command-Line Arguments
// ------------------------------------------------------------------------------------------------

/// Command-Line Interface (CLI) for the SDML language, primarily for verification and translation
/// from SDML source to other formats.
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
    /// Show tool and library versions.
    Version,
}

#[derive(Args, Debug)]
struct FileArgs {
    /// File name to write to, if not provided will write to stdout
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// A path to pre-pend to the resolver search path
    #[arg(short, long)]
    //#[clap(group = "resolver", requires = "module", conflicts_with="input_file")]
    base_path: Option<PathBuf>,

    /// SDML module, loaded using the standard resolver
    //#[clap(group = "resolver", conflicts_with="input_file")]
    module: Option<Identifier>,

    /// SDML File name, load without resolver
    //#[arg(short, long, conflicts_with = "resolver")]
    #[arg(short, long, conflicts_with = "module")]
    input_file: Option<PathBuf>,
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
    /// Format to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = TagFileFormat::CTags)]
    output_format: TagFileFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum TagFileFormat {
    /// ctags format
    CTags,
}

#[derive(Args, Debug)]
struct Convert {
    //> /// Configure the coloring of output
    //> #[structopt(
    //>     long = "color",
    //>     default_value = "auto",
    //>     possible_values = ColorArg::VARIANTS,
    //>     case_insensitive = true,
    //> )]
    //> pub color: ColorArg,
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
    //> /// Configure the coloring of output
    //> #[arg(
    //>     long = "color",
    //>     default_value = "auto",
    //>     possible_values = ColorArg::VARIANTS,
    //>     case_insensitive = true,
    //> )]
    //> pub color: ColorArg,
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

    debug!("arguments: {cli:#?}");

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
            Commands::Version => {
                println!("SDML CLI:        {}", CLI_VERSION);
                println!("SDML grammar:    {}", tree_sitter_sdml::GRAMMAR_VERSION);
                println!(
                    "Tree-Sitter ABI: {}",
                    tree_sitter_sdml::language().version()
                );
                Ok(())
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ File Args
// ------------------------------------------------------------------------------------------------

impl FileArgs {
    fn loader(&self) -> ModuleLoader {
        let resolver = ModuleResolver::default();
        if let Some(base) = &self.base_path {
            resolver.prepend_to_search_path(base)
        }
        ModuleLoader::from(resolver)
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
        let loader = self.files.loader();
        let resolver = loader.resolver();
        let source = if let Some(module_name) = &self.files.module {
            std::fs::read_to_string(resolver.name_to_path(module_name)?)?
        } else if let Some(module_file) = &self.files.input_file {
            std::fs::read_to_string(module_file)?
        } else {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut source = String::new();
            handle.read_to_string(&mut source)?;
            source
        };

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            HighlightFormat::Ansi => {
                sdml_generate::actions::highlight::write_highlighted_as_ansi(source, &mut writer)?
            }
            HighlightFormat::Html => sdml_generate::actions::highlight::write_highlighted_as_html(
                source,
                &mut writer,
                false,
            )?,
            HighlightFormat::HtmlStandalone => {
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
// Command Wrappers ❱ Tags
// ------------------------------------------------------------------------------------------------

impl Execute for Tags {
    fn execute(&self) -> Result<(), MainError> {
        let loader = self.files.loader();

        let file_name: PathBuf = if let Some(module_name) = &self.files.module {
            loader.resolver().name_to_path(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            module_file.clone()
        } else {
            unimplemented!("nope");
        };

        let model = loader.load_from_file(file_name.clone())?;
        let model = model.borrow();

        info!(
            "Loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            TagFileFormat::CTags =>
                sdml_generate::actions::tags::write_ctags(&model, file_name, &mut writer)?,
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Converter
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), MainError> {
        let mut loader = self.files.loader();
        let model = if let Some(module_name) = &self.files.module {
            loader.load(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            loader.load_from_file(module_file.clone())?
        } else {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            loader.load_from_reader(&mut handle)?
        };
        let model = model.borrow();

        info!(
            "loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            ConvertFormat::Org => {
                if let Some(path) = &self.files.output_file {
                    let mut generator: OrgFileGenerator =
                        sdml_generate::convert::org::OrgFileGenerator::default();
                    generator.write_to_file_in_format(
                        &model,
                        Some(&mut loader),
                        path,
                        Default::default(),
                    )?;
                } else {
                    let mut generator: OrgFileGenerator =
                        sdml_generate::convert::org::OrgFileGenerator::default();
                    generator.write_in_format(
                        &model,
                        Some(&mut loader),
                        &mut std::io::stdout(),
                        Default::default(),
                    )?;
                }
            }
            ConvertFormat::Rdf => {
                sdml_generate::convert::rdf::write_as_rdf(&model, &mut writer)?;
            }
            ConvertFormat::SExpr => {
                sdml_generate::convert::sexpr::write_as_sexpr(&model, &mut writer)?;
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
        let mut loader = self.files.loader();
        let model = if let Some(module_name) = &self.files.module {
            let loader = &mut loader;
            loader.load(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            let loader = &mut loader;
            loader.load_from_file(module_file.clone())?
        } else {
            let loader = &mut loader;
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            loader.load_from_reader(&mut handle)?
        };
        let model = model.borrow();

        info!(
            "loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        let format = self.output_format.clone().unwrap_or_default().into();

        info!("Generating {:?} diagram in {format:?} form.", self.diagram);

        match self.diagram {
            DrawDiagram::Concepts => {
                if let Some(path) = &self.files.output_file {
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default();
                    generator.write_to_file_in_format(&model, Some(&mut loader), path, format)?;
                } else {
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default();
                    generator.write_in_format(
                        &model,
                        Some(&mut loader),
                        &mut std::io::stdout(),
                        format,
                    )?;
                }
            }
            DrawDiagram::EntityRelationship => {
                if let Some(path) = &self.files.output_file {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    generator.write_to_file_in_format(&model, Some(&mut loader), path, format)?;
                } else {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    generator.write_in_format(
                        &model,
                        Some(&mut loader),
                        &mut std::io::stdout(),
                        format,
                    )?;
                }
            }
            DrawDiagram::UmlClass => {
                if let Some(path) = &self.files.output_file {
                    let mut generator = sdml_generate::draw::uml::UmlDiagramGenerator::default();
                    generator.write_to_file_in_format(&model, Some(&mut loader), path, format)?;
                } else {
                    panic!();
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
    type Err = sdml_core::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "org" => Ok(Self::Org),
            "rdf" => Ok(Self::Rdf),
            "s-expr" => Ok(Self::SExpr),
            _ => panic!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Kind
// ------------------------------------------------------------------------------------------------

// TODO: default?

//impl Display for DrawDiagram {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(
//            f,
//            "{}",
//            match self {
//                Self::Concepts => "concepts",
//                Self::EntityRelationship => "entity-relationship",
//                Self::UmlClass => "uml-class",
//            }
//        )
//    }
//}
//
//impl FromStr for DrawDiagram {
//    type Err = sdml_core::error::Error;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        match s {
//            "concepts" => Ok(Self::Concepts),
//            "entity-relationship" | "erd" => Ok(Self::EntityRelationship),
//            "uml-class" | "uml" => Ok(Self::UmlClass),
//            _ => panic!(),
//        }
//    }
//}

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
    type Err = sdml_core::error::Error;

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

//impl Display for DiagramFormat {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        write!(
//            f,
//            "{}",
//            match self {
//                Self::Source => "src",
//                Self::Jpeg => "jpg",
//                Self::Png => "png",
//                Self::Svg => "svg",
//            }
//        )
//    }
//}
//
//impl FromStr for DiagramFormat {
//    type Err = sdml_core::error::Error;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        match s {
//            "src" | "source" => Ok(Self::Source),
//            "jpg" | "jpeg" => Ok(Self::Jpeg),
//            "png" => Ok(Self::Png),
//            "svg" => Ok(Self::Svg),
//            _ => panic!(),
//        }
//    }
//}

impl From<DiagramFormat> for sdml_generate::draw::OutputFormat {
    fn from(v: DiagramFormat) -> Self {
        match v {
            DiagramFormat::Source => sdml_generate::draw::OutputFormat::Source,
            DiagramFormat::Jpeg => sdml_generate::draw::OutputFormat::ImageJpeg,
            DiagramFormat::Png => sdml_generate::draw::OutputFormat::ImagePng,
            DiagramFormat::Svg => sdml_generate::draw::OutputFormat::ImageSvg,
        }
    }
}
