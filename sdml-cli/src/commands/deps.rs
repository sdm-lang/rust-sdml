use clap::{Args, ValueEnum};
use sdml_core::{
    cache::{ModuleCache, ModuleStore},
    load::ModuleLoader,
};
use sdml_error::Error;
use sdml_generate::actions::deps::{DependencyViewGenerator, DependencyViewRepresentation};
use sdml_generate::GenerateToWriter;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Show a module's imported  dependencies
#[derive(Args, Debug)]
pub(crate) struct Command {
    #[arg(short = 'f', long)]
    #[arg(value_enum)]
    #[arg(default_value_t = OutputFormat::Tree)]
    output_format: OutputFormat,

    /// Depth to traverse imports, 0 implies all
    #[arg(short = 'd', long)]
    #[arg(default_value = "0")]
    depth: usize,

    #[command(flatten)]
    files: super::FileArgs,
}

/// The output format of the calculated dependencies
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum OutputFormat {
    /// GraphViz DOT format
    Graph,
    /// As RDF/OWL import triples
    Rdf,
    /// A hierarchical tree format
    Tree,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl super::Command for Command {
    fn execute(&self) -> Result<(), Error> {
        call_with_module!(self, |module, cache: &ModuleCache, _| {
            let mut generator = DependencyViewGenerator::new(self.depth);
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            generator.write_in_format(module, cache, &mut writer, self.output_format.into())
        });
    }
}

// ------------------------------------------------------------------------------------------------

impl From<OutputFormat> for DependencyViewRepresentation {
    fn from(v: OutputFormat) -> Self {
        match v {
            OutputFormat::Tree => DependencyViewRepresentation::TextTree,
            OutputFormat::Graph => DependencyViewRepresentation::DotGraph(Default::default()),
            OutputFormat::Rdf => DependencyViewRepresentation::RdfImports,
        }
    }
}
