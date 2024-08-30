use std::process::ExitCode;

use clap::{Args, ValueEnum};
use sdml_core::{
    load::ModuleLoader,
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::Error;
use sdml_generate::{
    actions::deps::{DependencyViewGenerator, DependencyViewOptions, DependencyViewRepresentation},
    Generator,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Show a module's imported  dependencies.
///
/// This command generates a representation of the transitive closure of dependencies for a given
/// module.
///
/// - Tree (default) :: Show dependencies as a text tree with the original as the root.
///
/// ```text
/// ❯ sdml deps sdml
/// sdml
/// ├── owl
/// │   ├── rdf
/// │   │   └── rdfs
/// │   │       └── rdf
/// │   ├── rdfs
/// │   └── xsd
/// │       ├── rdf
/// │       └── rdfs
/// ├── rdf
/// ├── rdfs
/// ├── skos
/// │   ├── rdf
/// │   └── rdfs
/// └── xsd
/// ```
///
/// - Graph :: Create an SVG representation of the dependency graph using GraphViz.
///
/// ```text
/// ❯ sdml deps --output-format graph -o sdml-deps.svg sdml
/// ❯ open -a Safari sdml-deps.svg
/// ```
///
/// - RDF :: Create a set of RDF statements,as N-Triples, that represent the individual OWL import relationships.
///
/// ```text
/// ❯ sdml deps --depth 1 --output-format rdf sdml
/// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2002/07/owl#> .
/// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
/// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2000/01/rdf-schema#> .
/// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2004/02/skos/core#> .
/// <http://sdml.io/sdml-owl.ttl#> <http://www.w3.org/2002/07/owl#imports> <http://www.w3.org/2001/XMLSchema#> .
/// ```
///
/// In some cases the entire set of dependencies is not necessary and the `--depth` argument can
/// be added to only show a number of levels of import from the root. The depth argument
/// instructs to command to stop after that many dependencies away from the original module.
/// Setting depth to 1 will only show the direct dependencies of the original.
///
/// ```text
/// ❯ sdml deps --depth 1 sdml
/// sdml
/// ├── owl
/// ├── rdf
/// ├── rdfs
/// ├── skos
/// └── xsd
/// ```
///
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
    #[allow(clippy::redundant_closure_call)]
    fn execute(&self) -> Result<ExitCode, Error> {
        call_with_module!(self, |module, cache: &InMemoryModuleCache, _| {
            let options: DependencyViewOptions = DependencyViewOptions::default()
                .with_depth(self.depth)
                .with_representation(self.output_format.into());
            let mut generator = DependencyViewGenerator::default();
            let mut output = self.files.output.clone();
            let mut writer = output.lock();

            generator.generate_with_options(module, cache, options, None, &mut writer)?;
            Ok(ExitCode::SUCCESS)
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
