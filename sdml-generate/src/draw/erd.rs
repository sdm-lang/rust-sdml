/*!
Provide a generator for Entity-Relationship diagrams via GraphViz.

*/

use crate::draw::{OutputFormat, DOT_PROGRAM};
use crate::exec::exec_with_temp_input;
use crate::GenerateToWriter;
use sdml_core::cache::ModuleCache;
use sdml_core::error::Error;
use sdml_core::model::identifiers::{Identifier, IdentifierReference};
use sdml_core::model::members::{Cardinality, TypeReference, DEFAULT_CARDINALITY};
use sdml_core::model::modules::{Import, Module};
use sdml_core::model::walk::{walk_module_simple, SimpleModuleWalker};
use sdml_core::model::{HasName, Span};
use std::io::Write;
use tracing::trace;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ErdDiagramGenerator {
    buffer: String,
    entity: Option<String>,
    seen: Vec<String>,
    format_options: OutputFormat,
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

impl GenerateToWriter<OutputFormat> for ErdDiagramGenerator {
    fn write<W>(
        &mut self,
        module: &Module,
        _cache: &ModuleCache,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        trace_entry!(
            "ErdDiagramGenerator",
            "write_in_format" =>
                "{}, _, _",
            module.name());
        walk_module_simple(module, self)?;

        let format = *self.format_options();
        if format == OutputFormat::Source {
            writer.write_all(self.buffer.as_bytes())?;
        } else {
            match exec_with_temp_input(DOT_PROGRAM, vec![format.into()], &self.buffer) {
                Ok(result) => {
                    writer.write_all(result.as_bytes())?;
                }
                Err(e) => {
                    panic!("exec_with_input failed: {:?}", e);
                }
            }
        }

        Ok(())
    }

    fn with_format_options(mut self, format_options: OutputFormat) -> Self {
        self.format_options = format_options;
        self
    }

    fn format_options(&self) -> &OutputFormat {
        &self.format_options
    }
}

impl SimpleModuleWalker for ErdDiagramGenerator {
    fn start_module(&mut self, _name: &Identifier, _: Option<&Span>) -> Result<(), Error> {
        self.buffer.push_str(
            r#"digraph G {
  bgcolor="transparent";
  rankdir="TB";
  fontname="Helvetica,Arial,sans-serif";
  node [fontname="Helvetica,Arial,sans-serif"; fontsize=10];
  edge [fontname="Helvetica,Arial,sans-serif"; fontsize=9; fontcolor="dimgrey";
        labelfontcolor="blue"; labeldistance=2.0];
  graph [pad="0.5", nodesep="1", ranksep="1"];
  splines="ortho";

"#,
        );
        Ok(())
    }

    fn import(&mut self, imported: &[Import], _: Option<&Span>) -> Result<(), Error> {
        trace!("import: {:?}", imported);
        for name in imported {
            self.buffer.push_str(&format!(
                "  {} [label=\"{}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
                name_to_ref(&name.to_string()),
                name
            ));
        }
        // ?        *self.entity.borrow_mut() = Some(name.to_string());
        Ok(())
    }

    fn start_entity(&mut self, name: &Identifier, _: bool, _: Option<&Span>) -> Result<(), Error> {
        trace!("entity: {}", name);
        self.buffer.push_str(&format!(
            "  {} [label=\"{}\"; penwidth=1.5];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_datatype(
        &mut self,
        name: &Identifier,
        _is_opaque: bool,
        _base_type: &IdentifierReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        trace!("datatype: {}", name);
        self.buffer.push_str(&format!(
            "  {} [label=\"■ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_enum(&mut self, name: &Identifier, _: bool, _: Option<&Span>) -> Result<(), Error> {
        trace!("enum: {}", name);
        self.buffer.push_str(&format!(
            "  {} [label=\"≣ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_event(
        &mut self,
        name: &Identifier,
        _source: &IdentifierReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        trace!("event: {}", name);
        self.buffer.push_str(&format!(
            "  {} [label=\"☇ {}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_structure(
        &mut self,
        name: &Identifier,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        trace!("structure: {}", name);
        self.buffer.push_str(&format!(
            "  {} [label=\"{}\"; style=\"dashed\"; color=\"dimgrey\"; fontcolor=\"dimgrey\"];\n",
            name_to_ref(name.as_ref()),
            name
        ));
        self.entity = Some(name.to_string());
        Ok(())
    }

    fn start_entity_identity(
        &mut self,
        name: &Identifier,
        target_type: &TypeReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        if matches!(target_type, TypeReference::Unknown)
            && !self.seen.contains(&"unknown".to_string())
        {
            self.buffer.push_str(
                "  unknown [shape=rect; label=\"Unknown\"; color=\"grey\"; fontcolor=\"grey\"];\n",
            );
            self.seen.push("unknown".to_string());
        }
        let target_type = if let TypeReference::Type(target_type) = target_type {
            name_to_ref(&target_type.to_string())
        } else {
            "unknown".to_string()
            // TODO: mapping types
        };
        self.buffer.push_str(&format!(
            "  {} -> {} [tooltip=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
            self.entity.as_deref().unwrap_or_default().to_lowercase(),
            target_type,
            name
        ));
        Ok(())
    }

    fn start_entity_identity_role_ref(
        &mut self,
        role_name: &Identifier,
        in_property: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            "  {} -> {} [label=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
            self.entity.as_deref().unwrap_or_default().to_lowercase(),
            in_property,
            role_name
        ));
        Ok(())
    }

    fn start_member(
        &mut self,
        name: &Identifier,
        inverse_name: Option<&Identifier>,
        target_cardinality: &Cardinality,
        target_type: &TypeReference,
        _: bool,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        let target_type = if let TypeReference::Type(target_type) = target_type {
            name_to_ref(&target_type.to_string())
            // TODO: mapping types
        } else {
            "unknown".to_string()
        };
        let target_cardinality = if *target_cardinality == DEFAULT_CARDINALITY {
            arrow_end("head", target_cardinality)
        } else {
            String::new()
        };
        self.buffer.push_str(&format!(
            "  {} -> {} [tooltip=\"{}\";dir=\"both\"{}{}];\n",
            self.entity.as_deref().unwrap_or_default().to_lowercase(),
            target_type,
            name,
            inverse_name.map(|id| id.to_string()).unwrap_or_default(),
            target_cardinality
        ));
        Ok(())
    }

    fn start_member_role_ref(
        &mut self,
        role_name: &Identifier,
        in_property: &IdentifierReference,
        _: Option<&Span>,
    ) -> Result<(), Error> {
        self.buffer.push_str(&format!(
            "  {} -> {} [label=\"{}\";dir=\"both\";arrowtail=\"teetee\";arrowhead=\"teetee\"];\n",
            self.entity.as_deref().unwrap_or_default().to_lowercase(),
            in_property,
            role_name
        ));
        Ok(())
    }

    fn end_module(&mut self, _: &Identifier) -> Result<(), Error> {
        self.buffer.push_str("}\n");
        self.entity = None;
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

const CARD_ONLY_ONE: &str = "teetee";
const CARD_ZERO_OR_ONE: &str = "teeodot";

const CARD_MANY: &str = "ocrow";
const CARD_ONE_OR_MANY: &str = "ocrowtee";
const CARD_ZERO_OR_MANY: &str = "ocrowodot";

#[inline(always)]
fn arrow_end(end: &str, cardinality: &Cardinality) -> String {
    format!(
        "; arrow{}=\"{}\"",
        end,
        match (cardinality.min_occurs(), cardinality.max_occurs()) {
            (0, None) => CARD_ZERO_OR_MANY,
            (1, None) => CARD_ONE_OR_MANY,
            (0, Some(1)) => CARD_ZERO_OR_ONE,
            (1, Some(1)) => CARD_ONLY_ONE,
            _ => CARD_MANY,
        }
    )
}

#[inline(always)]
fn name_to_ref(name: &str) -> String {
    name.replace(':', "__").to_lowercase()
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
