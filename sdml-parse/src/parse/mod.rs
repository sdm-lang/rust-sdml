/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::load::FsModuleLoader;
use crate::parse::modules::parse_module;
use sdml_core::load::ModuleLoader as ModuleLoaderTrait;
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::{Import, Module};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::NODE_KIND_MODULE;
use sdml_errors::diagnostics::functions::{
    duplicate_definition, duplicate_definition_import, duplicate_member, duplicate_module_import,
    duplicate_variant, found_error_node,
};
use sdml_errors::Error;
use sdml_errors::{FileId, Source};
use std::collections::HashSet;
use tracing::trace;
use tree_sitter::Node;
use tree_sitter::Parser;
use tree_sitter_sdml::language;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

#[macro_use]
mod macros;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// This should only be called by `ModuleLoader`
pub(crate) fn parse_str(file_id: FileId, loader: &FsModuleLoader) -> Result<Module, Error> {
    trace!("parse_str({file_id}, ...)");
    let file_cache = loader.files();
    let source = file_cache.get(file_id).unwrap().source();
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Error loading SDML grammar");

    let tree = parser.parse(source, None).unwrap();
    let node = tree.root_node();

    let mut context = ParseContext::new(file_id, loader);
    context.check_if_error(&node, "module")?;

    if node.kind() == NODE_KIND_MODULE {
        let mut cursor = tree.walk();
        let mut module = parse_module(&mut context, &mut cursor)?;
        module.set_file_id(file_id);
        Ok(module)
    } else {
        unexpected_node!(context, "parse_str", node, NODE_KIND_MODULE);
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct ParseContext<'a> {
    loader: &'a FsModuleLoader,
    file_id: FileId,
    source: Source,
    imports: HashSet<Import>,
    type_names: HashSet<Identifier>,
    member_names: HashSet<Identifier>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<'a> ParseContext<'a> {
    fn new(file_id: FileId, loader: &'a FsModuleLoader) -> Self {
        let file_cache = loader.files();
        let file = file_cache.get(file_id).unwrap();
        Self {
            file_id,
            loader,
            source: file.source().clone(),
            imports: Default::default(),
            type_names: Default::default(),
            member_names: Default::default(),
        }
    }

    fn node_source(&'a self, node: &'a Node<'a>) -> Result<&'a str, Error> {
        Ok(node.utf8_text(self.source.as_ref())?)
    }

    fn check_if_error(&self, node: &Node<'a>, rule: &str) -> Result<(), Error> {
        if node.is_error() {
            let diagnostic = found_error_node(self.file_id, node.byte_range(), rule);
            self.loader.report(&diagnostic).unwrap();
            Err(diagnostic.into())
        } else {
            Ok(())
        }
    }

    fn add_import(&mut self, import: &Import) -> Result<(), Error> {
        if let Some(previous) = self.imports.get(import) {
            let diagnostic = if matches!(previous, Import::Module(_)) {
                duplicate_module_import(
                    self.file_id,
                    previous.source_span().unwrap().byte_range(),
                    import.source_span().unwrap().byte_range(),
                )
            } else {
                duplicate_definition_import(
                    self.file_id,
                    previous.source_span().unwrap().byte_range(),
                    import.source_span().unwrap().byte_range(),
                )
            };
            self.loader.report(&diagnostic).unwrap();
        } else {
            self.imports.insert(import.clone());
        }
        Ok(())
    }

    fn start_type(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(type_defn) = self.type_names.get(name) {
            let diagnostic = duplicate_definition(
                self.file_id,
                name.source_span().unwrap().byte_range(),
                type_defn.source_span().unwrap().byte_range(),
            );
            self.loader.report(&diagnostic).unwrap();
        } else {
            self.type_names.insert(name.clone());
        }
        Ok(())
    }

    fn start_member(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(member) = self.member_names.get(name) {
            let diagnostic = duplicate_member(
                self.file_id,
                member.source_span().unwrap().byte_range(),
                name.source_span().unwrap().byte_range(),
            );
            self.loader.report(&diagnostic).unwrap();
        } else {
            self.member_names.insert(name.clone());
        }
        Ok(())
    }

    fn start_variant(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(member) = self.member_names.get(name) {
            let diagnostic = duplicate_variant(
                self.file_id,
                member.source_span().unwrap().byte_range(),
                name.source_span().unwrap().byte_range(),
            );
            self.loader.report(&diagnostic).unwrap();
        } else {
            self.member_names.insert(name.clone());
        }
        Ok(())
    }

    fn end_type(&mut self) {
        self.member_names.clear()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod modules;

mod identifiers;

mod annotations;

mod definitions;

mod members;

mod values;

mod constraints;
