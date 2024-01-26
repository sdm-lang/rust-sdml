use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use sdml_core::cache::ModuleCache;
use sdml_core::error::{tracing_filter_error, tracing_subscriber_error};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_core::stdlib;
use sdml_generate::{GenerateToFile, GenerateToWriter};
use sdml_parse::load::{ModuleLoader, ModuleResolver};
use std::io::Read;
use std::path::PathBuf;
use tracing::info;
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
    /// View formatted module source code
    View(View),
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
struct View {
    /// Format to convert into
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = SourceGenerationLevel::Full)]
    level: SourceGenerationLevel,

    /// Format to convert into
    #[arg(short = 's', long)]
    #[arg(default_value = "2")]
    indent_spaces: usize,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SourceGenerationLevel {
    /// Full source
    Full,
    /// Top-level definitions
    Definitions,
    /// Top-level definitions and member definitions
    Members,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Deps {
    /// Format to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DepsFormat::Tree)]
    output_format: DepsFormat,

    /// Depth to traverse imports, 0 implies all
    #[arg(short = 'd', long)]
    #[arg(default_value = "0")]
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
            Commands::View(cmd) => cmd.execute(),
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
            if stdlib::is_library_module(module_name) {
                println!("Sorry, can't currently highlight stdlib modules");
                return Ok(());
            } else {
                std::fs::read_to_string(resolver.name_to_path(module_name)?)?
            }
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
// Command Wrappers ❱ Macros
// ------------------------------------------------------------------------------------------------

macro_rules! call_with_module {
    ($cmd: expr, $callback_fn: expr) => {
        let (module_name, cache, loader) = {
            let mut cache = ModuleCache::default().with_stdlib();
            let mut loader = $cmd.files.loader();
            let module_name = if let Some(module_name) = &$cmd.files.module {
                loader.load(module_name, &mut cache, true)?
            } else if let Some(file_name) = &$cmd.files.input_file {
                loader.load_from_file(file_name.clone(), &mut cache, true)?
            } else {
                let stdin = std::io::stdin();
                let mut handle = stdin.lock();
                loader.load_from_reader(&mut handle, &mut cache, true)?
            };
            (module_name, cache, loader)
        };
        let module = cache
            .get(&module_name)
            .expect("Error: module not found in cache");
        let is_complete = module.is_complete(&cache)?;
        let is_valid = module.is_valid(false, &cache)?;
        info!(
            "Loaded module: {}, is_complete: {is_complete}, is_valid: {is_valid}",
            module.name(),
        );
        return $callback_fn(module, &cache, &loader);
    };
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Dependencies
// ------------------------------------------------------------------------------------------------

impl Execute for Deps {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(self, |module, cache: &ModuleCache, _| {
            let mut writer = self.files.output_writer()?;

            match self.output_format {
                DepsFormat::Tree => sdml_generate::actions::deps::write_dependency_tree(
                    module,
                    cache,
                    self.depth,
                    &mut writer,
                )?,
                DepsFormat::Graph => sdml_generate::actions::deps::write_dependency_graph(
                    module,
                    cache,
                    self.depth,
                    &mut writer,
                )?,
                DepsFormat::Rdf => sdml_generate::actions::deps::write_dependency_rdf(
                    module,
                    cache,
                    self.depth,
                    &mut writer,
                )?,
            }

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Document
// ------------------------------------------------------------------------------------------------

impl Execute for Doc {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(
            self,
            |module: &Module, cache: &ModuleCache, loader: &ModuleLoader| {
                match self.output_format {
                    DocFormat::OrgMode => {
                        let source = loader.get_source(module.name());
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
                    DocFormat::Markdown => {}
                }

                Ok(())
            }
        );
    }
}

impl Doc {
    fn write_org(
        &self,
        model: &Module,
        cache: &ModuleCache,
        generator: &mut sdml_generate::convert::doc::org_mode::DocumentationGenerator,
    ) -> Result<(), MainError> {
        if let Some(path) = &self.files.output_file {
            generator.write_to_file_in_format(model, cache, path, Default::default())?;
        } else {
            generator.write_in_format(model, cache, &mut std::io::stdout(), Default::default())?;
        }
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Tags
// ------------------------------------------------------------------------------------------------

impl Execute for Tags {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(self, |module: &Module, _, _| {
            let mut writer = self.files.output_writer()?;

            match self.output_format {
                TagFileFormat::CTags => sdml_generate::actions::tags::write_ctags(
                    module,
                    module.source_file().cloned().unwrap_or_default(),
                    &mut writer,
                )?,
            }

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Converter
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(self, |module: &Module, cache: &ModuleCache, _| {
            let mut writer = self.files.output_writer()?;

            match self.output_format {
                ConvertFormat::Rdf => {
                    let mut generator = sdml_generate::convert::rdf::RdfModelGenerator::default();
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(
                            module,
                            cache,
                            path,
                            Default::default(),
                        )?;
                    } else {
                        generator.write_in_format(
                            module,
                            cache,
                            &mut std::io::stdout(),
                            Default::default(),
                        )?;
                    }
                }
                ConvertFormat::SExpr => {
                    sdml_generate::convert::sexpr::write_as_sexpr(module, &mut writer)?;
                }
            }

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Draw
// ------------------------------------------------------------------------------------------------

impl Execute for Draw {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(self, |module: &Module, cache: &ModuleCache, _| {
            let format = self.output_format.clone().unwrap_or_default().into();

            info!("Generating {:?} diagram in {format:?} form.", self.diagram);

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
// Command Wrappers ❱ View
// ------------------------------------------------------------------------------------------------

impl Execute for View {
    fn execute(&self) -> Result<(), MainError> {
        call_with_module!(self, |module: &Module, cache, _| {
            let mut writer = self.files.output_writer()?;

            let mut generator = sdml_generate::convert::source::SourceGenerator::default();
            generator.write_in_format(module, cache, &mut writer, self.level.clone().into())?;

            Ok(())
        });
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

// ------------------------------------------------------------------------------------------------

impl From<SourceGenerationLevel> for sdml_generate::convert::source::SourceGenerationLevel {
    fn from(v: SourceGenerationLevel) -> Self {
        match v {
            SourceGenerationLevel::Full => {
                sdml_generate::convert::source::SourceGenerationLevel::Full
            }
            SourceGenerationLevel::Definitions => {
                sdml_generate::convert::source::SourceGenerationLevel::Definitions
            }
            SourceGenerationLevel::Members => {
                sdml_generate::convert::source::SourceGenerationLevel::Members
            }
        }
    }
}
