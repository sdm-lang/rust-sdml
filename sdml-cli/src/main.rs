use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use sdml_core::cache::ModuleCache;
use sdml_core::error::{tracing_filter_error, tracing_subscriber_error};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_generate::{GenerateToFile, GenerateToWriter};
use sdml_parse::load::{load_module_dependencies, ModuleLoader, ModuleResolver};
use std::io::Read;
use std::path::PathBuf;
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

// ------------------------------------------------------------------------------------------------
// Command-Line Arguments
// ------------------------------------------------------------------------------------------------

// TODO: Consider adding build.rs for man-file generation using https://crates.io/crates/clap_mangen

// TODO: Add support for external sub-commands https://docs.rs/clap/latest/clap/struct.Command.html#method.allow_external_subcommands

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

// TODO: Consider using crate https://docs.rs/clap_complete to add completion generation.

#[derive(Subcommand, Debug)]
enum Commands {
    /// Document a module
    Doc(Doc),
    /// Show module dependencies
    Deps(Deps),
    /// Extract tags from a module
    Tags(Tags),
    /// Syntax highlight a module source
    Highlight(Highlight),
    /// Convert a module into other formats
    Convert(Convert),
    /// Draw diagrams from a module
    Draw(Draw),
    /// Show tool and library versions.
    Version,
}

// ------------------------------------------------------------------------------------------------
// TODO: Consider using crate https://docs.rs/clio instead

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

// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Deps {
    /// Format to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DepsFormat::Tree)]
    output_format: DepsFormat,

    /// Depth to traverse imports
    #[arg(short = 'd', long)]
    depth: usize,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum DepsFormat {
    /// hierarchical tree format
    Tree,
    /// GraphViz DOT format
    Graph,
    /// as RDF/OWL import triples
    Rdf,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Doc {
    /// Format to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DocFormat::OrgMode)]
    output_format: DocFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
enum DocFormat {
    /// Emacs org-mode
    OrgMode,
    /// Markdown
    Markdown,
}

// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------

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
    /// RDF Abstract Model
    Rdf,
    /// S-Expressions
    SExpr,
}

// ------------------------------------------------------------------------------------------------

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
            Commands::Doc(cmd) => cmd.execute(),
            Commands::Deps(cmd) => cmd.execute(),
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
        let mut resolver = ModuleResolver::default();
        if let Some(base) = &self.base_path {
            resolver.prepend_to_search_path(base)
        }
        ModuleLoader::default().with_resolver(resolver)
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
// Command Wrappers ❱ Dependencies
// ------------------------------------------------------------------------------------------------

impl Execute for Deps {
    fn execute(&self) -> Result<(), MainError> {
        let mut cache = ModuleCache::default().with_stdlib();
        let mut loader = self.files.loader();

        let file_name: PathBuf = if let Some(module_name) = &self.files.module {
            loader.resolver().name_to_path(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            module_file.clone()
        } else {
            unimplemented!("nope");
        };

        let module = loader.load_from_file(file_name.clone())?;

        info!(
            "Loaded module: {}, is_complete: {:?}",
            module.name(),
            module.is_complete()
        );

        if self.depth > 1 {
            load_module_dependencies(&module, true, &mut cache, &mut loader)?;
        }

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            DepsFormat::Tree => sdml_generate::actions::deps::write_dependency_tree(
                &module,
                &cache,
                self.depth,
                &mut writer,
            )?,
            DepsFormat::Graph => sdml_generate::actions::deps::write_dependency_graph(
                &module,
                &cache,
                self.depth,
                &mut writer,
            )?,
            DepsFormat::Rdf => sdml_generate::actions::deps::write_dependency_rdf(
                &module,
                &cache,
                self.depth,
                &mut writer,
            )?,
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Document
// ------------------------------------------------------------------------------------------------

impl Execute for Doc {
    fn execute(&self) -> Result<(), MainError> {
        let mut loader = self.files.loader();

        let file_name: PathBuf = if let Some(module_name) = &self.files.module {
            loader.resolver().name_to_path(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            module_file.clone()
        } else {
            unimplemented!("nope");
        };

        let model = loader.load_from_file(file_name.clone())?;

        info!(
            "Loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        match self.output_format {
            DocFormat::OrgMode => {
                let source = loader.get_source(model.name());
                if let Some(source) = source {
                    let mut generator =
                        sdml_generate::convert::org::OrgFileGenerator::with_source(source.as_ref());
                    self.write_org(&model, &mut generator)?;
                } else {
                    let mut generator = sdml_generate::convert::org::OrgFileGenerator::default();
                    self.write_org(&model, &mut generator)?;
                };
            }
            DocFormat::Markdown => {}
        }

        Ok(())
    }
}

impl Doc {
    fn write_org(
        &self,
        model: &Module,
        generator: &mut sdml_generate::convert::org::OrgFileGenerator,
    ) -> Result<(), MainError> {
        if let Some(path) = &self.files.output_file {
            generator.write_to_file_in_format(model, path, Default::default())?;
        } else {
            generator.write_in_format(model, &mut std::io::stdout(), Default::default())?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Tags
// ------------------------------------------------------------------------------------------------

impl Execute for Tags {
    fn execute(&self) -> Result<(), MainError> {
        let mut loader = self.files.loader();

        let file_name: PathBuf = if let Some(module_name) = &self.files.module {
            loader.resolver().name_to_path(module_name)?
        } else if let Some(module_file) = &self.files.input_file {
            module_file.clone()
        } else {
            unimplemented!("nope");
        };

        let model = loader.load_from_file(file_name.clone())?;

        info!(
            "Loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        match self.output_format {
            TagFileFormat::CTags => {
                sdml_generate::actions::tags::write_ctags(&model, file_name, &mut writer)?
            }
        }

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Converter
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), MainError> {
        //let cache = ModuleCache::default().with_stdlib();
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

        info!(
            "loaded module: {}, is_complete: {:?}",
            model.name(),
            model.is_complete()
        );

        let mut writer = self.files.output_writer()?;

        match self.output_format {
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
        let mut cache = ModuleCache::default().with_stdlib();
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

        load_module_dependencies(&model, true, &mut cache, &mut loader)?;

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
                    generator.write_to_file_in_format(&model, path, format)?;
                } else {
                    let mut generator =
                        sdml_generate::draw::concepts::ConceptDiagramGenerator::default();
                    generator.write_in_format(&model, &mut std::io::stdout(), format)?;
                }
            }
            DrawDiagram::EntityRelationship => {
                if let Some(path) = &self.files.output_file {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    generator.write_to_file_in_format(&model, path, format)?;
                } else {
                    let mut generator = sdml_generate::draw::erd::ErdDiagramGenerator::default();
                    generator.write_in_format(&model, &mut std::io::stdout(), format)?;
                }
            }
            DrawDiagram::UmlClass => {
                if let Some(path) = &self.files.output_file {
                    let mut generator = sdml_generate::draw::uml::UmlDiagramGenerator::default();
                    generator.write_to_file_in_format(&model, path, format)?;
                } else {
                    println!("Sorry, writing UML diagrams requires an explicit output file");
                }
            }
        }

        Ok(())
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

// ------------------------------------------------------------------------------------------------

impl Default for DiagramFormat {
    fn default() -> Self {
        Self::Source
    }
}

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
