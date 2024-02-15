use clap::builder::FalseyValueParser;
use clap::{Args, Parser, Subcommand, ValueEnum};
use sdml_core::cache::ModuleCache;
use sdml_core::load::{ModuleLoader, ModuleResolver};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::Module;
use sdml_core::model::HasName;
use sdml_core::stdlib;
use sdml_error::diagnostics::{Reporter, StandardStreamReporter, set_diagnostic_level_filter, SeverityFilter};
use sdml_error::Error;
use sdml_generate::{GenerateToFile, GenerateToWriter};
use sdml_parse::load::{FsModuleLoader, FsModuleResolver};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;
use tracing::{error, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter as TracingLevelFilter;
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
    /// Level of logging to enable
    #[arg(long)]
    #[arg(value_enum)]
    #[arg(default_value_t = LogFilter::None)]
    log_filter: LogFilter,

    /// Turn off color for code emitters
    #[arg(
        long,
        global = true,
        action = clap::ArgAction::SetTrue,
        env = "NO_COLOR",
        value_parser = FalseyValueParser::new(),
    )]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum LogFilter {
    /// Turn off all logging
    None,
    /// Enable error logging only
    Errors,
    /// Enable warnings and above
    Warnings,
    /// Enable information and above
    Information,
    /// Enable debugging and above
    Debugging,
    /// Enable tracing (ALL) and above
    Tracing,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Validate a module
    Validate(Validate),
    /// Document a module
    Doc(Doc),
    /// Show module dependencies
    Deps(Deps),
    /// Extract tags from a module
    Tags(Tags),
    /// Syntax highlight a module source
    Highlight(Highlight),
    /// Convert module into alternate representations
    Convert(Convert),
    /// Draw diagrams from a module
    Draw(Draw),
    /// View formatted module source code
    View(View),
    /// Show tool and library versions.
    Versions,
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
    #[clap(group = "resolver", requires = "module", conflicts_with = "input_file")]
    base_path: Option<PathBuf>,

    /// SDML module, loaded using the standard resolver
    #[clap(
        group = "resolver",
        conflicts_with="input_file",
        value_parser = Identifier::from_str)]
    module: Option<Identifier>,

    /// SDML File name, load without resolver
    #[arg(short, long, conflicts_with = "resolver")]
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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
struct Validate {
    /// Set the level of diagnostic messages to report
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DiagnosticLevel::Warnings)]
    level: DiagnosticLevel,

    /// Enable the checking of constraints in the model
    #[arg(short = 'c', long, default_value = "false")]
    check_constraints: bool,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticLevel {
    /// Turn off reporting
    None,
    /// Implementation bugs
    Bugs,
    /// Module errors
    Errors,
    /// Module warnings
    Warnings,
    /// Module informational notes
    Notes,
    /// Style and other help
    Help,
    /// Turn it all on
    All,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct View {
    /// Determine the amount of source to generate
    #[arg(short = 'l', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = SourceGenerationLevel::Full)]
    level: SourceGenerationLevel,

    /// Set the number of spaces for indentation
    #[arg(short = 's', long)]
    #[arg(default_value = "2")]
    indent_spaces: usize,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SourceGenerationLevel {
    /// Top-level definitions, incomplete
    Definitions,
    /// Top-level definitions and members, incomplete
    Members,
    /// Full source
    Full,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Deps {
    /// The format of dependency data to produce
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value = "tree")]
    output_format: DepsFormat,

    /// Depth to traverse imports, 0 implies all
    #[arg(short = 'd', long)]
    #[arg(default_value = "0")]
    depth: usize,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum DepsFormat {
    /// A hierarchical tree format
    Tree,
    /// GraphViz DOT format
    Graph,
    /// As RDF/OWL import triples
    Rdf,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Doc {
    /// Documentation format to generate
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = DocFormat::OrgMode)]
    output_format: DocFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum TagFileFormat {
    /// ctags format
    CTags,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Convert {
    /// Module representation to convert into
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    output_format: ConvertFormat,

    #[command(flatten)]
    files: FileArgs,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum ConvertFormat {
    /// RDF Abstract Model
    Rdf,
    /// JSON
    Json,
    /// Pretty-printed JSON
    JsonPretty,
    /// S-Expressions
    SExpr,
}

// ------------------------------------------------------------------------------------------------

#[derive(Args, Debug)]
struct Draw {
    /// Diagram kind to draw
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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum DrawDiagram {
    /// Concept Overview
    Concepts,
    /// Entity Relationship
    EntityRelationship,
    /// UML Class
    UmlClass,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum DiagramFormat {
    Source,
    Jpeg,
    Png,
    Svg,
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! call_with_module {
    ($cmd: expr, $callback_fn: expr) => {
        let (module_name, cache, loader) = {
            let mut cache = ModuleCache::default().with_stdlib();
            let mut loader = $cmd.files.loader();
            let module_name = if let Some(module_name) = &$cmd.files.module {
                loader.load(
                    module_name,
                    loader.get_file_id(&module_name),
                    &mut cache,
                    true,
                )?
            } else if let Some(file_name) = &$cmd.files.input_file {
                match loader.load_from_file(file_name.clone(), &mut cache, true) {
                    Err(e) => {
                        println!(
                            "Error: the input file `{}` does not exist.",
                            file_name.display()
                        );
                        return Err(e);
                    }
                    Ok(loaded) => loaded,
                }
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
        return $callback_fn(module, &cache, &loader);
    };
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() -> ExitCode {
    human_panic::setup_panic!();

    let cli = Cli::parse();

    init_color(cli.no_color);

    if let Err(e) = init_logging(cli.log_filter) {
        error!("init_logging failed, exiting. error: {e:?}");
        ExitCode::FAILURE
    } else {
        if let Err(e) = cli.command.execute() {
            error!("command.execute failed, exiting. error: {e:?}");
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Main ❱ Color
// ------------------------------------------------------------------------------------------------

fn init_color(no_color: bool) {
    if no_color {
        sdml_generate::color::set_colorize(sdml_generate::color::UseColor::Never);
    }
}

// ------------------------------------------------------------------------------------------------
// Main ❱ Logging
// ------------------------------------------------------------------------------------------------

fn init_logging(log_filter: LogFilter) -> Result<(), Error> {
    let log_level_filter = match log_filter {
        LogFilter::None => TracingLevelFilter::OFF,
        LogFilter::Errors => TracingLevelFilter::ERROR,
        LogFilter::Warnings => TracingLevelFilter::WARN,
        LogFilter::Information => TracingLevelFilter::INFO,
        LogFilter::Debugging => TracingLevelFilter::DEBUG,
        LogFilter::Tracing => TracingLevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env().add_directive(
        format!("{}={}", module_path!(), log_level_filter)
            .parse()
            .map_err(sdml_error::Error::from)?,
    );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).map_err(sdml_error::Error::from)?;

    info!("Log level set to `LevelFilter::{:?}`", log_filter);

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers
// ------------------------------------------------------------------------------------------------

trait Execute {
    fn execute(&self) -> Result<(), Error>;
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Router
// ------------------------------------------------------------------------------------------------

impl Execute for Commands {
    fn execute(&self) -> Result<(), Error> {
        match self {
            Commands::Highlight(cmd) => cmd.execute(),
            Commands::Doc(cmd) => cmd.execute(),
            Commands::Deps(cmd) => cmd.execute(),
            Commands::Tags(cmd) => cmd.execute(),
            Commands::Convert(cmd) => cmd.execute(),
            Commands::Draw(cmd) => cmd.execute(),
            Commands::View(cmd) => cmd.execute(),
            Commands::Validate(cmd) => cmd.execute(),
            Commands::Versions => {
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
    fn loader(&self) -> FsModuleLoader {
        let mut resolver = FsModuleResolver::default();
        if let Some(base) = &self.base_path {
            resolver.prepend_to_search_path(base)
        }
        FsModuleLoader::default().with_resolver(resolver)
    }

    fn output_writer(&self) -> Result<Box<dyn std::io::Write>, Error> {
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
    fn execute(&self) -> Result<(), Error> {
        let loader = self.files.loader();
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
        } else if let Some(module_file) = &self.files.input_file {
            if module_file.is_file() {
                std::fs::read_to_string(module_file)?
            } else {
                println!(
                    "Error: the input file `{}` does not exist.",
                    module_file.display()
                );
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
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
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module, cache: &ModuleCache, _| {
            let mut writer = self.files.output_writer()?;

            let mut generator =
                sdml_generate::actions::deps::DependencyViewGenerator::new(self.depth);
            generator.write_in_format(
                module,
                cache,
                &mut writer,
                self.output_format.into(),
            )?;

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Document
// ------------------------------------------------------------------------------------------------

impl Execute for Doc {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(
            self,
            |module: &Module, cache: &ModuleCache, loader: &FsModuleLoader| {
                match self.output_format {
                    DocFormat::OrgMode => {
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
    ) -> Result<(), Error> {
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
    fn execute(&self) -> Result<(), Error> {
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
// Command Wrappers ❱ Representation Generator
// ------------------------------------------------------------------------------------------------

impl Execute for Convert {
    fn execute(&self) -> Result<(), Error> {
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
                ConvertFormat::Json | ConvertFormat::JsonPretty => {
                    let mut generator = sdml_generate::convert::json::Generator::default();
                    let options = if self.output_format == ConvertFormat::JsonPretty {
                        sdml_generate::convert::json::GeneratorOptions::pretty_printer()
                    } else {
                        sdml_generate::convert::json::GeneratorOptions::default()
                    };
                    if let Some(path) = &self.files.output_file {
                        generator.write_to_file_in_format(module, cache, path, options)?;
                    } else {
                        generator.write_in_format(
                            module,
                            cache,
                            &mut std::io::stdout(),
                            options,
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
    fn execute(&self) -> Result<(), Error> {
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
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module: &Module, cache, _| {
            let mut writer = self.files.output_writer()?;

            let mut generator = sdml_generate::convert::source::SourceGenerator::default();
            generator.write_in_format(module, cache, &mut writer, self.level.clone().into())?;

            Ok(())
        });
    }
}

// ------------------------------------------------------------------------------------------------
// Command Wrappers ❱ Validate
// ------------------------------------------------------------------------------------------------

impl Execute for Validate {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module: &Module, cache, loader| {

            set_diagnostic_level_filter(self.level.into()).unwrap();

            let reporter = StandardStreamReporter::default();

            module.validate(cache, loader, self.check_constraints);
            reporter.done(Some(module.name().to_string()))?;

            Ok(())
        });
    }
}

impl From<DiagnosticLevel> for SeverityFilter {
    fn from(value: DiagnosticLevel) -> Self {
        match value
 {
    DiagnosticLevel::None => SeverityFilter::None,
    DiagnosticLevel::Bugs => SeverityFilter::Bug,
    DiagnosticLevel::Errors => SeverityFilter::Error,
    DiagnosticLevel::Warnings => SeverityFilter::Warning,
    DiagnosticLevel::Notes => SeverityFilter::Note,
    DiagnosticLevel::Help => SeverityFilter::Help,
    DiagnosticLevel::All => SeverityFilter::Help,
 }
    }
}

// ------------------------------------------------------------------------------------------------
// Formats ❱ Diagram Format
// ------------------------------------------------------------------------------------------------

impl From<DepsFormat> for sdml_generate::actions::deps::DependencyViewRepresentation {
    fn from(v: DepsFormat) -> Self {
        match v {
            DepsFormat::Tree => {
                sdml_generate::actions::deps::DependencyViewRepresentation::TextTree
            }
            DepsFormat::Graph => {
                sdml_generate::actions::deps::DependencyViewRepresentation::DotGraph
            }
            DepsFormat::Rdf => {
                sdml_generate::actions::deps::DependencyViewRepresentation::RdfImports
            }
        }
    }
}

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
