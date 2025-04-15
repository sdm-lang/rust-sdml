use clap::{Args, Subcommand};
use sdml_core::model::identifiers::Identifier;
use sdml_errors::Error;
use std::{process::ExitCode, str::FromStr};
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public-ish Macros
// ------------------------------------------------------------------------------------------------

macro_rules! call_with_module {
    ($cmd: expr, $callback_fn: expr) => {
        let reporter = ::sdml_errors::diagnostics::StandardStreamReporter::default();
        call_with_module!($cmd, Box::new(reporter), $callback_fn);
    };
    ($cmd: expr, $reporter:expr, $callback_fn: expr) => {
        let (module_name, cache, mut loader) = {
            let mut cache = ::sdml_core::store::InMemoryModuleCache::default();
            let mut loader = ::sdml_parse::load::FsModuleLoader::default().with_reporter($reporter);
            let module_name = if let Some(module_name) = &$cmd.files.module {
                loader.load(
                    module_name,
                    loader.get_file_id(&module_name),
                    &mut cache,
                    true,
                )?
            } else if $cmd.files.input.is_local() {
                let file_name = $cmd.files.input.path();
                match loader.load_from_file(file_name.to_path_buf(), &mut cache, true) {
                    Err(::sdml_errors::Error::LanguageValidationError { source: _ }) => {
                        loader.reporter_done(None)?;
                        return Ok(ExitCode::FAILURE);
                    }
                    Err(err @ ::sdml_errors::Error::IoError { source: _ }) => {
                        println!(
                            "Error: the input file `{}` could not be found, or read.",
                            file_name.display()
                        );
                        return Err(err);
                    }
                    Err(err) => {
                        loader.reporter_done(None)?;
                        return Err(err);
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
        return $callback_fn(module, &cache, &mut loader);
    };
}

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) trait Command {
    fn execute(&self) -> Result<ExitCode, Error>;
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Convert(convert::Command),
    Deps(deps::Command),
    Doc(doc::Command),
    DocBook(book::Command),
    Draw(draw::Command),
    Generate(generate::Command),
    Highlight(highlight::Command),
    Tags(tags::Command),
    Validate(validate::Command),
    Versions(versions::Command),
    View(view::Command),
}

#[derive(Args, Debug)]
pub(crate) struct FileArgs {
    /// File name to write to, or '-' to write to stdout
    #[arg(short, long)]
    #[clap(value_parser, default_value = "-")]
    output: clio::Output,

    /// SDML module, loaded using the standard resolver
    #[clap(
        group = "resolver",
        conflicts_with="input",
        value_parser = Identifier::from_str)]
    module: Option<Identifier>,

    /// Input SDML file name to read from, or '-' to read from stdin
    #[arg(short, long)]
    #[clap(value_parser, default_value = "-", conflicts_with = "resolver")]
    input: clio::Input,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Command for Commands {
    fn execute(&self) -> Result<ExitCode, Error> {
        trace!("Commands::execute self: {self:?}");
        match self {
            Commands::Highlight(cmd) => cmd.execute(),
            Commands::Generate(cmd) => cmd.execute(),
            Commands::Doc(cmd) => cmd.execute(),
            Commands::DocBook(cmd) => cmd.execute(),
            Commands::Deps(cmd) => cmd.execute(),
            Commands::Tags(cmd) => cmd.execute(),
            Commands::Convert(cmd) => cmd.execute(),
            Commands::Draw(cmd) => cmd.execute(),
            Commands::View(cmd) => cmd.execute(),
            Commands::Validate(cmd) => cmd.execute(),
            Commands::Versions(cmd) => cmd.execute(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod book;
mod convert;
mod deps;
mod doc;
mod draw;
mod generate;
mod highlight;
mod tags;
mod validate;
mod versions;
mod view;
