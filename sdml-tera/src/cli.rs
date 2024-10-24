use clap::Parser;
use sdml_core::load::ModuleLoader;
use sdml_core::model::identifiers::Identifier;
use sdml_core::store::InMemoryModuleCache;
use sdml_core::store::ModuleStore;
use sdml_errors::diagnostics::StandardStreamReporter;
use sdml_errors::Error;
use sdml_parse::load::FsModuleLoader;
use sdml_tera::context::module_to_value;
use sdml_tera::make_engine_from;
use sdml_tera::render_module_to;
use sdml_tera::render_module_to_file;
use serde_json::{from_reader, Value};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::process::ExitCode;
use tera::Context;

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! error_message {
    ($msg: expr, $err: expr) => {
        eprintln!("{}.\nError: {}\n", $msg, $err);
        return ExitCode::FAILURE;
    };
    ($msg: expr) => {
        eprintln!("{}.", $msg);
        return ExitCode::FAILURE;
    };
}

macro_rules! handle_render {
    ($result: expr) => {
        match $result {
            Err(err) => {
                error_message!(
                    "An error occurred rendering the model with the named template; most likely you referenced an unknown value",
                    err
                );
            }
            _ => {}
        }
    };
}

macro_rules! handle_reporter {
    ($loader: expr) => {
        if let Err(err) = $loader.reporter_done(None) {
            error_message!("An error occurred closing down the reporter", err);
        }
    };
}

macro_rules! handle_loader {
    ($result: expr, $source: expr, $loader: expr) => {
        match $result {
            Err(Error::LanguageValidationError { source: _ }) => {
                handle_reporter!($loader);
                return ExitCode::FAILURE;
            }
            Err(Error::IoError { source: err }) => {
                error_message!(
                    format!("An error occurred loading the module from the {}", $source),
                    err
                );
            }
            Err(err) => {
                handle_reporter!($loader);
                error_message!("An unknown error occurred during module loading", err);
            }
            Ok(loaded) => loaded,
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// SDML module for context, loaded using the standard resolver
    #[clap(value_parser(clap::value_parser!(Identifier)), group = "input")]
    module_name: Option<Identifier>,

    /// Input SDML file name to read from, or '-' to read from stdin
    #[arg(short = 'i', long)]
    #[clap(
        value_parser,
        value_name = "FILE",
        default_value = "-",
        group = "input"
    )]
    module_input: clio::Input,

    /// Load context from FILE before adding any module
    #[arg(long, value_name = "FILE")]
    context: Option<PathBuf>,

    /// Do not merge the module into the loaded context
    #[arg(long, action, requires = "context")]
    no_merge: bool,

    /// Display the computed context prior to the template
    #[arg(long, action)]
    context_debug: bool,

    /// A glob expression denoting the templates to load
    #[arg(short = 'g', long)]
    #[clap(value_parser, value_name = "GLOB", default_value = "templates/**/*.md")]
    template_glob: String,

    /// The name of a template loaded from GLOB
    #[arg(short = 'n', long)]
    #[clap(value_parser)]
    template_name: String,

    /// File name to write to, or '-' to write to stdout
    #[arg(short = 'o', long)]
    #[clap(value_parser, value_name = "FILE", default_value = "-")]
    output: clio::Output,
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() -> ExitCode {
    human_panic::setup_panic!();

    let cli = Cli::parse();

    let reporter = StandardStreamReporter::default();
    let mut cache = InMemoryModuleCache::default().with_stdlib();
    let mut loader = FsModuleLoader::default().with_reporter(Box::new(reporter));

    let module_name = if let Some(module_name) = &cli.module_name {
        handle_loader!(
            loader.load(
                module_name,
                loader.get_file_id(module_name),
                &mut cache,
                true,
            ),
            "file system",
            loader
        )
    } else if cli.module_input.is_local() {
        let file_name = cli.module_input.path();
        handle_loader!(
            loader.load_from_file(file_name.to_path_buf(), &mut cache, true),
            format!("the file {}", file_name.display()),
            loader
        )
    } else {
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle_loader!(
            loader.load_from_reader(&mut handle, &mut cache, true),
            "the standard input stream",
            loader
        )
    };

    let module = match cache.get(&module_name) {
        None => {
            error_message!("An error occurred fetching the loaded module from the load cache");
        }
        Some(module) => module,
    };

    let engine = match make_engine_from(&cli.template_glob) {
        Err(err) => {
            error_message!(
                "An error occurred creating the Tera engine; most likely this is a syntax error in one of your templates",
                err
            );
        }
        Ok(engine) => engine,
    };

    let context: Option<Context> = if let Some(context_file) = cli.context {
        let file = OpenOptions::new().read(true).open(&context_file).unwrap();
        let context: Value = from_reader(file).unwrap();
        Some(Context::from_value(context).unwrap())
    } else {
        None
    };

    if cli.context_debug {
        let mut context = context.clone().unwrap_or_default();
        let (_, value) = module_to_value(module, &cache);
        context.insert("module", &value);
        let stdout = std::io::stdout().lock();
        serde_json::to_writer_pretty(stdout, &context.into_json()).unwrap();
        println!("");
    }

    if cli.output.is_local() {
        handle_render!(render_module_to_file(
            &engine,
            module,
            &cache,
            context,
            &cli.template_name,
            cli.output.path().path(),
        ));
    } else {
        handle_render!(render_module_to(
            &engine,
            module,
            &cache,
            context,
            &cli.template_name,
            &mut std::io::stdout(),
        ));
    }

    ExitCode::SUCCESS
}
