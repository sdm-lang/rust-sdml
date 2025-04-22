/*!
One-line description.

More detailed description, with

# Example

 */

use crate::write::to_graph::{Context, ToGraph};
use rdftk_core::model::graph::{Graph, PrefixMapping};
use rdftk_io::{
    self, nq::NQuadWriter, nt::NTripleWriter, nt::NTripleWriterOptions, turtle::TurtleWriter,
    HasOptions, ObjectWriter,
};
use sdml_core::{
    model::modules::Module,
    repr::RepresentationWriter,
    store::{InMemoryModuleCache, ModuleStore},
};
use sdml_errors::Error as ApiError;
use std::{fmt::Display, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct Options {
    include_source_location: bool,
    mappings: Option<PrefixMapping>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Dialect {
    NTriples,
    #[default]
    Turtle,
    NQuads,
    TriX,
}

#[derive(Clone, Debug, Default)]
pub struct WriterOptions {
    dialect: Dialect,
    options: Options,
}

#[derive(Clone, Debug, Default)]
pub struct Writer;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn module_to_graph(
    module: &Module,
    cache: &impl ModuleStore,
    options: &Options,
) -> Result<Graph, ApiError> {
    let mut ctx = Context::from(module)?.with_options(options.clone());
    module.to_graph(&mut ctx, cache)
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Writer
// ------------------------------------------------------------------------------------------------

impl RepresentationWriter for Writer {
    type Object = Module;
    type Cache = InMemoryModuleCache;
    type Options = WriterOptions;

    fn write_with<W>(
        &self,
        w: &mut W,
        module: &Self::Object,
        cache: Option<&Self::Cache>,
        options: &Self::Options,
    ) -> Result<(), ApiError>
    where
        W: Write,
    {
        let WriterOptions { dialect, options } = options;
        let graph = module_to_graph(module, cache.unwrap(), options)?;
        match dialect {
            Dialect::NTriples => NTripleWriter::default()
                .with_options(NTripleWriterOptions::default().force_string_literals(true))
                .write(w, &graph),
            Dialect::Turtle => TurtleWriter::default().write(w, &graph),
            Dialect::NQuads => NQuadWriter::default().write(w, &graph),
            Dialect::TriX => todo!(),
        }
        .map_err(|e| ApiError::GeneratorError {
            name: dialect.to_string(),
            message: e.to_string(),
        })
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Dialect
// ------------------------------------------------------------------------------------------------

impl Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NTriples => "RDF NTriples",
                Self::Turtle => "RDF Turtle",
                Self::NQuads => "RDF NQuads",
                Self::TriX => "RDF TRiX",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Public Implementations ❱ Options
// ------------------------------------------------------------------------------------------------

impl Options {
    pub fn with_include_source_location(self, include_source_location: bool) -> Self {
        let mut self_mut = self;
        self_mut.set_include_source_location(include_source_location);
        self_mut
    }

    pub fn include_source_location(&self) -> bool {
        self.include_source_location
    }

    pub fn set_include_source_location(&mut self, include_source_location: bool) {
        self.include_source_location = include_source_location
    }

    pub fn with_mappings(self, mappings: PrefixMapping) -> Self {
        let mut self_mut = self;
        self_mut.set_mappings(mappings);
        self_mut
    }

    pub fn mappings(&self) -> Option<&PrefixMapping> {
        self.mappings.as_ref()
    }

    pub fn set_mappings(&mut self, mappings: PrefixMapping) {
        self.mappings = Some(mappings);
    }

    pub fn unset_mappings(&mut self) {
        self.mappings = None;
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

pub mod to_graph;
