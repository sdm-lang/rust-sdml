/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{
    ErrorCounters, FileId, MEMBER_ALREADY_IMPORTED, MEMBER_NAME_USED, MODULE_ALREADY_IMPORTED,
    TYPE_DEFINITION_NAME_USED,
};
use crate::load::{ModuleLoader, Source};
use crate::parse::modules::parse_module;
use codespan_reporting::diagnostic::Label;
use sdml_core::error::{module_parse_error, Error};
use sdml_core::model::identifiers::Identifier;
use sdml_core::model::modules::{Import, Module};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::NODE_KIND_MODULE;
use std::collections::HashSet;
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

#[derive(Clone, Debug)]
pub(crate) struct Parsed {
    module: Module,
    counters: ErrorCounters,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// This should only be called by `ModuleLoader`
pub(crate) fn parse_str(file_id: FileId, loader: &ModuleLoader) -> Result<Parsed, Error> {
    let file_cache = loader.files();
    let source = file_cache.get(file_id).unwrap().source();
    let mut parser = Parser::new();
    parser
        .set_language(language())
        .expect("Error loading SDML grammar");
    let tree = parser.parse(source, None).unwrap();

    let node = tree.root_node();
    let mut context = ParseContext::new(file_id, loader);

    if node.kind() == NODE_KIND_MODULE {
        let mut cursor = tree.walk();
        let module = parse_module(&mut context, &mut cursor)?;
        Ok(Parsed {
            module,
            counters: context.counts,
        })
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
    loader: &'a ModuleLoader,
    file_id: FileId,
    source: Source,
    imports: HashSet<Import>,
    type_names: HashSet<Identifier>,
    member_names: HashSet<Identifier>,
    counts: ErrorCounters,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Parsed {
    pub(crate) fn into_inner(self) -> (Module, ErrorCounters) {
        (self.module, self.counters)
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> ParseContext<'a> {
    fn new(file_id: FileId, loader: &'a ModuleLoader) -> Self {
        let file_cache = loader.files();
        let file = file_cache.get(file_id).unwrap();
        Self {
            file_id,
            loader,
            source: file.source().clone(),
            imports: Default::default(),
            type_names: Default::default(),
            member_names: Default::default(),
            counts: Default::default(),
        }
    }

    fn node_source(&'a self, node: &'a Node<'a>) -> Result<&'a str, Error> {
        Ok(node.utf8_text(self.source.as_ref())?)
    }

    fn check_if_error(&self, node: &Node<'a>, rule: &str) -> Result<(), Error> {
        if node.is_error() {
            Err(module_parse_error(node.kind(), node.into(), Some(rule)))
        } else {
            Ok(())
        }
    }

    fn add_import(&mut self, import: &Import) -> Result<(), Error> {
        if let Some(previous) = self.imports.get(import) {
            let diagnostic = if matches!(previous, Import::Module(_)) {
                MEMBER_ALREADY_IMPORTED
            } else {
                MODULE_ALREADY_IMPORTED
            }
            .into_diagnostic()
            .with_labels(vec![
                Label::primary(self.file_id, import.source_span().unwrap().byte_range())
                    .with_message("this module"),
                Label::secondary(self.file_id, previous.source_span().unwrap().byte_range())
                    .with_message("was initially imported here"),
            ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
        } else {
            self.imports.insert(import.clone());
        }
        Ok(())
    }

    fn start_type(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(type_defn) = self.type_names.get(name) {
            let diagnostic = TYPE_DEFINITION_NAME_USED
                .into_diagnostic()
                .with_labels(vec![
                    Label::primary(self.file_id, name.source_span().unwrap().byte_range())
                        .with_message("this type name"),
                    Label::secondary(self.file_id, type_defn.source_span().unwrap().byte_range())
                        .with_message("was previously defined here"),
                ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
        } else {
            self.type_names.insert(name.clone());
        }
        Ok(())
    }

    fn start_member(&mut self, name: &Identifier) -> Result<(), Error> {
        if let Some(member) = self.member_names.get(name) {
            let diagnostic = MEMBER_NAME_USED.into_diagnostic().with_labels(vec![
                Label::primary(self.file_id, name.source_span().unwrap().byte_range())
                    .with_message("this member name"),
                Label::secondary(self.file_id, member.source_span().unwrap().byte_range())
                    .with_message("was previously defined here"),
            ]);

            self.counts.report(diagnostic.severity);
            emit_diagnostic!(self.loader.files(), diagnostic);
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

fn message_found_node(found: &str) -> String {
    format!("found `{found}`")
}

fn message_expecting_node(expecting: &str) -> String {
    format!("expecting: `{expecting}`")
}

fn message_expecting_one_of_node(expecting: &[&str]) -> String {
    format!(
        "expecting one of: {}",
        expecting
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<String>>()
            .join("|")
    )
}

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
