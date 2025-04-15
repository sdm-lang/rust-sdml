use clap::Args;
use sdml_core::store::InMemoryModuleCache;
use sdml_errors::{diagnostics::StandardStreamReporter, Error};
use sdml_generate::convert::doc::{
    org_mode::DocumentationGenerator, BookConfig, DocumentationWriter,
};
use sdml_parse::load::FsModuleLoader;
use std::{path::PathBuf, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Produce structured documentation for a set of modules.
///
#[derive(Args, Debug)]
pub(crate) struct Command {
    /// Path to the doc-book configuration file.
    #[arg(short = 'c', long)]
    #[arg(default_value = "doc-book.json")]
    config_file: PathBuf,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<ExitCode, Error> {
        let config = BookConfig::from_file(&self.config_file)?;
        let mut generator = DocumentationGenerator::default();

        let reporter = StandardStreamReporter::default();
        let mut loader = FsModuleLoader::default().with_reporter(Box::new(reporter));
        let mut cache = InMemoryModuleCache::default();

        generator.write_book(&mut loader, &mut cache, config)?;

        Ok(ExitCode::SUCCESS)
    }
}
